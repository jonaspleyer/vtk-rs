use std::collections::BTreeMap;

use crate::{
    Result,
    parse_wrap_vtk_xml::{Class, Method, Module},
};

pub type ClassName = String;

pub struct ClassHierarchy {
    /// Contains (class_name, (module_name, class))
    pub classes: BTreeMap<ClassName, Class>,
    pub tree: BTreeMap<ClassName, (Class, Vec<ClassName>)>,
    pub dependents: BTreeMap<ClassName, Vec<ClassName>>,
}

pub fn type_regex() -> regex::Regex {
    regex::Regex::new(r#"([a-zA-Z0-9:]*)(<[.*]>)?"#).unwrap()
}

impl ClassHierarchy {
    pub fn new(modules: &[Module]) -> Result<Self> {
        let mut errors = vec![];
        let classes: BTreeMap<String, Class> = modules
            .iter()
            .flat_map(|m| m.files.iter().flat_map(|file| file.1.classes.iter()))
            .fold(BTreeMap::new(), |mut acc, class| {
                acc.entry(class.name.clone())
                    .and_modify(|entry| {
                        if let Err(e) = entry.combine(class) {
                            errors.push(e);
                        }
                    })
                    .or_insert(class.clone());

                acc
            });

        if let Some(e) = errors.into_iter().next() {
            Err(e)?;
        }

        let mut dependents = BTreeMap::<ClassName, Vec<ClassName>>::new();
        let tree: BTreeMap<_, _> = classes
            .iter()
            .map(|(class_name, class)| {
                // Obtain all classes from which the currently selected class inherits
                let parents = class
                    .inheritance
                    .as_ref()
                    .map(|x| x.context.clone())
                    .unwrap_or_default()
                    .iter()
                    .map(|context| {
                        // Deterct generic type arguments
                        let re = type_regex();
                        let name_reduced = &re.captures(&context.name).unwrap()[0];
                        name_reduced.to_string()
                    })
                    .collect::<Vec<_>>();
                parents.iter().for_each(|parent| {
                    dependents
                        .entry(parent.clone())
                        .and_modify(|x: &mut Vec<_>| x.push(class_name.clone()))
                        .or_insert(vec![class_name.clone()]);
                });
                Ok((class_name.clone(), (class.clone(), parents)))
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            classes,
            tree,
            dependents,
        })
    }

    pub fn get_parent_names(&self, class_name: &str) -> impl IntoIterator<Item = &String> {
        self.tree
            .get(class_name)
            .into_iter()
            .flat_map(|(_, names)| names)
    }

    pub fn get_parents(&self, class_name: &str) -> impl IntoIterator<Item = &Class> {
        self.get_parent_names(class_name)
            .into_iter()
            .filter_map(|name| self.classes.get(name))
    }

    /// Returns true if `target` appears anywhere in the full ancestor chain of `class_name`.
    pub fn has_ancestor(&self, class_name: &str, target: &str) -> bool {
        let mut stack = vec![class_name.to_string()];
        let mut visited = std::collections::HashSet::new();
        while let Some(current) = stack.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }
            if let Some((_, parents)) = self.tree.get(&current) {
                for parent in parents {
                    if parent == target {
                        return true;
                    }
                    stack.push(parent.clone());
                }
            }
        }
        false
    }

    pub fn has_dependant(&self, class: &Class) -> bool {
        self.dependents
            .get(&class.name.clone())
            .is_some_and(|x| !x.is_empty())
    }

    /// This function returns a list of methods which can be used to define a trait for this class.
    ///
    /// In particular we filter for the following
    /// 1. public methods
    /// 2. non-inherited methods
    /// 3. non-generic (no templates)
    /// 4. any method containing the keyword
    ///    [`typename`](https://en.cppreference.com/w/cpp/keyword/typename.html)
    pub fn get_exposable_methods(&self, class_name: &str) -> Result<Vec<Method>> {
        let mut all_parents: Vec<ClassName> = self
            .tree
            .get(class_name)
            .map(|x| x.1.clone())
            .unwrap_or_default();

        let mut remaining_parents = all_parents.clone();

        while let Some(class_name) = remaining_parents.pop() {
            if let Some((_, nodes)) = self.tree.get(&class_name.clone()) {
                remaining_parents.extend(nodes.clone());
                all_parents.extend(nodes.clone());
            }
        }

        let parent_methods: Vec<_> = all_parents
            .into_iter()
            .filter_map(|class_name| self.classes.get(&class_name))
            .flat_map(|c| c.methods.public.iter())
            .collect();

        let unique_methods: Vec<_> = self.classes[class_name]
            .methods
            .public
            .clone()
            .into_iter()
            .filter(|x| !parent_methods.contains(&x))
            .filter(|x| x.signature.trim().chars().take(8).collect::<String>() != "template")
            .filter(|x| !x.signature.contains("typename"))
            // C-style array parameters (e.g. `double pts[3]`) decay to pointers but WrapVTK
            // records no pointer attribute for them. We can't bridge array parameters safely
            // without explicit length info, so skip any method whose signature contains `[`.
            .filter(|x| !x.signature.contains('['))
            .collect();

        Ok(unique_methods)
    }
}

#[cfg(test)]
mod inheritance_tests {
    use super::*;
    use crate::parse_wrap_vtk_xml::{
        Access, CContext, Class, Constructor, Destructor, File, Inheritance, Method, Methods,
        Module,
    };

    fn make_method(name: &str) -> Method {
        Method {
            name: name.to_string(),
            property: None,
            access: Access::Public,
            is_const: false,
            is_static: false,
            is_virtual: true,
            signature: format!("void {}()", name),
            parameters: vec![],
            comment: None,
            return_type: None,
        }
    }

    fn make_class(name: &str, parents: Vec<&str>, methods: Vec<&str>) -> Class {
        Class {
            name: name.to_string(),
            is_abstract: false,
            is_template: false,
            comment: None,
            base: vec![],
            inheritance: if parents.is_empty() {
                None
            } else {
                Some(Inheritance {
                    context: parents
                        .iter()
                        .map(|p| CContext {
                            name: p.to_string(),
                            access: Access::Public,
                        })
                        .collect(),
                })
            },
            methods: Methods {
                public: methods.iter().map(|m| make_method(m)).collect(),
                private: vec![],
                protected: vec![],
            },
            typedefs: vec![],
            properties: vec![],
            members: vec![],
            constructors: vec![Constructor {
                access: Access::Public,
                signature: String::new(),
            }],
            destructors: vec![Destructor {
                access: Access::Public,
                signature: String::new(),
            }],
        }
    }

    fn make_module(classes: Vec<Class>) -> Module {
        Module {
            name: "TestModule".to_string(),
            path: std::path::PathBuf::new(),
            files: vec![(
                std::path::PathBuf::new(),
                File {
                    name: "test.h".to_string(),
                    classes,
                },
            )],
        }
    }

    #[test]
    fn test_get_parent_names_direct_parent() {
        let base = make_class("Base", vec![], vec!["BaseMethod"]);
        let child = make_class("Child", vec!["Base"], vec!["ChildMethod"]);
        let module = make_module(vec![base, child]);
        let hierarchy = ClassHierarchy::new(&[module]).unwrap();

        let parents: Vec<_> = hierarchy.get_parent_names("Child").into_iter().collect();
        assert_eq!(parents, vec!["Base"]);
    }

    #[test]
    fn test_get_parent_names_root_class_has_no_parents() {
        let base = make_class("Base", vec![], vec!["BaseMethod"]);
        let module = make_module(vec![base]);
        let hierarchy = ClassHierarchy::new(&[module]).unwrap();

        let parents: Vec<_> = hierarchy.get_parent_names("Base").into_iter().collect();
        assert!(parents.is_empty());
    }

    #[test]
    fn test_get_exposable_methods_unique_to_child() {
        let base = make_class("Base", vec![], vec!["SharedMethod"]);
        let child = make_class("Child", vec!["Base"], vec!["SharedMethod", "ChildOnly"]);
        let module = make_module(vec![base, child]);
        let hierarchy = ClassHierarchy::new(&[module]).unwrap();

        let methods = hierarchy.get_exposable_methods("Child").unwrap();
        let names: Vec<_> = methods.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["ChildOnly"]);
    }

    #[test]
    fn test_get_exposable_methods_all_when_no_parent() {
        let base = make_class("Base", vec![], vec!["Method1", "Method2"]);
        let module = make_module(vec![base]);
        let hierarchy = ClassHierarchy::new(&[module]).unwrap();

        let methods = hierarchy.get_exposable_methods("Base").unwrap();
        assert_eq!(methods.len(), 2);
    }

    #[test]
    fn test_get_exposable_methods_filters_template_signatures() {
        let mut class = make_class("Foo", vec![], vec![]);
        class.methods.public.push(Method {
            name: "TplMethod".to_string(),
            signature: "template <typename T> void TplMethod()".to_string(),
            ..make_method("TplMethod")
        });
        class.methods.public.push(make_method("NormalMethod"));
        let module = make_module(vec![class]);
        let hierarchy = ClassHierarchy::new(&[module]).unwrap();

        let methods = hierarchy.get_exposable_methods("Foo").unwrap();
        let names: Vec<_> = methods.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["NormalMethod"]);
    }
}

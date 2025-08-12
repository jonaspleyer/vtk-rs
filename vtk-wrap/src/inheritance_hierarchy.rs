use std::collections::HashMap;

use crate::{
    Result,
    parse_wrap_vtk_xml::{Class, Method, Module},
};

type ClassName = String;

pub struct ClassHierarchy {
    /// Contains (class_name, (module_name, class))
    pub classes: HashMap<ClassName, Class>,
    pub tree: HashMap<ClassName, (Class, Vec<ClassName>)>,
    pub dependents: HashMap<ClassName, Vec<ClassName>>,
}

pub fn type_regex() -> regex::Regex {
    regex::Regex::new(r#"([a-zA-Z0-9:]*)(<[.*]>)?"#).unwrap()
}

impl ClassHierarchy {
    pub fn new(modules: &[Module]) -> Result<Self> {
        let mut errors = vec![];
        let classes: HashMap<String, Class> = modules
            .iter()
            .flat_map(|m| m.files.iter().flat_map(|file| file.1.classes.iter()))
            .fold(HashMap::new(), |mut acc, class| {
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

        let mut dependents = HashMap::<ClassName, Vec<ClassName>>::new();
        let tree: HashMap<_, _> = classes
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

    pub fn has_dependant(&self, class: &Class) -> bool {
        self.dependents
            .get(&class.name.clone())
            .is_some_and(|x| !x.is_empty())
    }

    pub fn get_non_inherited_public_methods(&self, class_name: &str) -> Result<Vec<Method>> {
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
            .flat_map(|class_name| self.classes[&class_name].methods.public.iter())
            .collect();

        let unique_methods: Vec<_> = self.classes[class_name]
            .methods
            .public
            .clone()
            .into_iter()
            .filter(|x| !parent_methods.contains(&x))
            .collect();

        Ok(unique_methods)
    }
}

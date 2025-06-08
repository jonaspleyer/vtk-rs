use std::collections::HashMap;

use crate::{
    Result,
    parsing::{Class, Method, Module},
};

#[derive(Hash, PartialEq, Eq)]
pub struct ClassNode {
    name: ClassName,
}

type ClassName = String;

pub struct Hierarchy {
    /// Contains (class_name, (module_name, class))
    classes: HashMap<ClassName, Class>,
    tree: HashMap<ClassNode, (Class, Vec<ClassNode>)>,
}

impl Hierarchy {
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
                        let re = regex::Regex::new(r#"([a-zA-Z0-9:]*)(<[.*]>)?"#).unwrap();
                        let name_reduced = &re.captures(&context.name).unwrap()[0];
                        ClassNode {
                            name: name_reduced.to_string(),
                        }
                    })
                    .collect::<Vec<_>>();
                Ok((
                    ClassNode {
                        name: class_name.clone(),
                    },
                    (class.clone(), parents),
                ))
            })
            .collect::<Result<_>>()?;

        Ok(Self { classes, tree })
    }

    pub fn get_non_inherited_public_methods(&self, class_name: &str) -> Result<Vec<Method>> {
        let mut all_parents: Vec<ClassName> = self
            .tree
            .get(&ClassNode {
                name: class_name.to_string(),
            })
            .map(|x| x.1.iter().map(|node| node.name.clone()).collect())
            .unwrap_or_default();

        let mut remaining_parents = all_parents.clone();

        while let Some(class_name) = remaining_parents.pop() {
            if let Some((_, nodes)) = self.tree.get(&ClassNode {
                name: class_name.clone(),
            }) {
                let new_names = nodes
                    .iter()
                    .map(|node| node.name.clone())
                    .collect::<Vec<_>>();

                remaining_parents.extend(new_names.clone());
                all_parents.extend(new_names);
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

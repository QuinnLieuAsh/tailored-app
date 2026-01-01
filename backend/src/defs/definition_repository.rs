use std::collections::HashMap;
use std::vec::Vec;

use super::definitions::Definition;

pub trait DefinitionRepository {

    //REQUIRES:
    //MODIFIES:
    //EFFECTS:
    fn create(&mut self, term: &str, def: &str) -> Definition;

    //REQUIRES:
    //MODIFIES:
    //EFFECTS: Returns the name of term removed
    fn remove(&mut self, id: &str) -> Option<Definition>;

    //REQUIRES:
    //MODIFIES:
    //EFFECTS: Returns a list of Defitions in order of recency
    fn list(&self) -> Vec<Definition>;

    //REQUIRES: ID to be existing in DB
    //MODIFIES:
    //EFFECTS: Returns a Definition with id matching the argument
    fn get_by_id(&self, id: &str) -> Option<Definition>;

    //REQUIRES: Term to be existing in DB
    //MODIFIES:
    //EFFECTS: Returns a Definition with term matching the argument
    fn get_by_term(&self, term: &str) -> Option<Definition>;

}


pub struct DefinitionManager {
    def_map: HashMap<String, Definition>
}

impl DefinitionManager {

    pub fn new() -> Self {
        Self {
            def_map: HashMap::new()
        }
    }
}

impl DefinitionRepository for DefinitionManager {

    fn create(&mut self, term: &str, def: &str) -> Definition {
        let definition: Definition = Definition::new(term, def);
        let return_def = definition.clone();
        self.def_map.insert(definition.id.clone(), definition);
        return_def
    }

    fn get_by_id(&self, id: &str) -> Option<Definition> {
        self.def_map.get(id).cloned()
    }

    fn get_by_term(&self, term: &str) -> Option<Definition> {
        self.def_map.values()
        .find(|def| def.term == term).cloned()
    }

    fn list(&self) -> Vec<Definition> {
       self.def_map.values().cloned().collect()
    }

    fn remove(&mut self, id: &str) -> Option<Definition> {
        self.def_map.remove(id)
        
    }
    
}


use std::collections::HashMap;

// Note struct
pub struct Note {
    pub(crate) matiere: String,
    pub(crate) notes: HashMap<String, i32>,
    pub(crate) moyenne: f32,
    pub(crate) coef: i32,
}

// Student struct

pub struct Student {
    prenom: String,
    nom: String,
    age: i32,
    classe: String,
    notes: Vec<Note>,
    is_active: bool,
}

// Student implementation
impl Student {
    pub(crate) fn new(prenom: String, nom: String, age: i32, classe: String, notes: Vec<Note>, is_active: bool) -> Student {
        return Student {
            prenom,
            nom,
            age,
            classe,
            notes,
            is_active,
        };
    }

    pub(crate) fn get_full_name(&self) -> String {
        return format!("{} {}", self.prenom, self.nom);
    }

    pub(crate) fn get_age(&self) -> i32 {
        return self.age;
    }

    pub(crate) fn get_classe(&self) -> String {
        return self.classe.to_string();
    }

    pub(crate) fn get_notes(&self) -> Vec<Note> {
        return self.notes.iter().clone().map(|note| {
            Note {
                matiere: note.matiere.clone(),
                notes: note.notes.clone(),
                moyenne: note.moyenne,
                coef: note.coef,
            }
        }).collect();
        //return self.notes.clone();
    }

    pub(crate) fn get_is_active(&self) -> bool {
        return self.is_active;
    }

    pub(crate) fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    fn get_total_coef(&self) -> i32 {
        let mut total_coef: i32 = 0;
        for note in self.notes.iter() {
            total_coef = total_coef + note.coef
        }
        return total_coef;
    }

    pub(crate) fn get_moyenne(&self) -> f32 {
        let mut moyenne: f32 = 0.0;
        let total_coef: i32 = self.get_total_coef();
        for note in self.notes.iter() {
            moyenne = moyenne + (note.moyenne * note.coef as f32);
        }
        return moyenne/total_coef as f32;
    }
}
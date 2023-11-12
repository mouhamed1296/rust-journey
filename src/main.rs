mod student;
use std::collections::HashMap;

/*fn main() {
    println!("Hello, world!");
    let mut u: i32 = 1;
    // Loop
    while u < 10 {
        u = u + 1;
        if u%2 == 0 {
            println!("u est paire");
            continue;
        }
        println!("La valeur de u est: {}", u);
        let sum = add(1 , 2);
        // Formatting
        println!("La somme de {} + {} est: {}", 1, 2, sum);
        let user: HashMap<String, String> = dictionary();
        println!("Le nom de l'utilisateur est: {}", get_map_value(user, String::from("name")));
        // Tuple
        let (x, y) = tuple();
        println!("x = {}, y = {}", x, y);
        // Array
        let arr = [1, 2, 3, 4, 5];
        println!("La valeur de l'index 0 est: {}", arr[0]);
        // Loop
        for i in 0..5 {
            println!("La valeur de l'index {} est: {}", i, arr[i]);
        }
        // Loop
        for i in arr.iter() {
            println!("La valeur de l'index est: {}", i);
        }
        // Loop
        for (i, j) in arr.iter().enumerate() {
            println!("La valeur de l'index {} est: {}", i, j);
        }
        // Loop
        for i in 0..5 {
            if i == 2 {
                println!("La valeur de i est: {}", i);
                break;
            }
        }
    }
    println!("Fin du programme");
}*/
/*
// Function to add two numbers
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// create a function that returns a tuple
fn tuple() -> (i32, i32) {
    return (1, 2);
}

// create a function that returns a dictionary
fn dictionary() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(String::from("name"), String::from("John"));
    map.insert(String::from("age"), String::from("20"));
    map.insert(String::from("city"), String::from("New York"));
    return map;
}

fn get_map_value(map: HashMap<String, String>, key: String) -> String {
    return map.get(&key).unwrap_or(&String::from("")).to_string();
}*/


fn create_student() -> student::Student {
    println!("********** Création d'un étudiant **********");
    println!("Entrez le prénom de l'étudiant: ");
    let mut prenom: String = String::new();
    std::io::stdin().read_line(&mut prenom).expect("Erreur lors de la lecture de la valeur");
    println!("Entrez le nom de l'étudiant: ");
    let mut nom: String = String::new();
    std::io::stdin().read_line(&mut nom).expect("Erreur lors de la lecture de la valeur");
    let mut classe: String = String::new();
    println!("Veuillez entrer la classe de l'étudiant");
    std::io::stdin().read_line(&mut classe).expect("Erreur lors de la lecture de la valeur");
    let mut notes: Vec<student::Note> = Vec::new();
    let mut notes_maths: HashMap<String, i32> = HashMap::new();
    let moyenne_maths: f32;
    notes_maths.insert(String::from("DS1"), 10);
    notes_maths.insert(String::from("DS2"), 15);
    notes_maths.insert(String::from("DS3"), 20);
    moyenne_maths = ((notes_maths.get("DS1").unwrap_or(&0)
        + notes_maths.get("DS2").unwrap_or(&0)
        + notes_maths.get("DS3").unwrap_or(&0)) / 3) as f32;
    notes.push(student::Note {
        matiere: String::from("Maths"),
        notes: notes_maths,
        moyenne: moyenne_maths,
        coef: 3,
    });
    let mut notes_physique: HashMap<String, i32> = HashMap::new();
    notes_physique.insert(String::from("DS1"), 10);
    notes_physique.insert(String::from("DS2"), 15);
    notes_physique.insert(String::from("DS3"), 20);
    notes.push(student::Note {
        matiere: String::from("Physique"),
        notes: notes_physique,
        moyenne: 15.0,
        coef: 3,
    });
    let mut notes_anglais: HashMap<String, i32> = HashMap::new();
    notes_anglais.insert(String::from("DS1"), 10);
    notes_anglais.insert(String::from("DS2"), 15);
    notes_anglais.insert(String::from("DS3"), 20);
    notes.push(student::Note {
        matiere: String::from("Anglais"),
        notes: notes_anglais,
        moyenne: 15.0,
        coef: 3,
    });
    return student::Student::new(prenom, nom, 20, classe, notes, true);
}

fn main() {
    let mut student: student::Student = create_student();
    println!("********** Student **********");
    println!("Nom complet: {}", student.get_full_name());
    println!("Age: {}", student.get_age());
    println!("Classe: {}", student.get_classe());
    println!("********** Notes **********");
    for note in student.get_notes().iter() {
        println!("Matiere: {}", note.matiere);
        println!("Notes: {:?}", note.notes);
        println!("Moyenne: {}", note.moyenne);
        println!("Coef: {}", note.coef);
    }
    println!("********** Moyenne **********");
    println!("Moyenne: {}", student.get_moyenne());
    student.set_is_active(false);
    println!("Is active: {}", student.get_is_active());
}
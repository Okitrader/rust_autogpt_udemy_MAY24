trait Attacker {
    fn chose_style(&self) -> String;
    fn chose_weapon(&self) -> String;
}
enum Character {
    Warrior,
    Archer,
    Wizard
}

impl Attacker for Character {
    fn chose_style(&self) -> String {
        match self { // all the possible of the enum.
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "kung fu".to_string(),
            Character::Wizard => "thai chi".to_string(),
        }
    }

    fn chose_weapon(&self) -> String {
        match self {
            Character::Warrior => "Sword".to_string(),
            Character::Archer => "Bow".to_string(),
            Character::Wizard => "Staff".to_string(),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_traits() {
        let my_character: Character = Character::Archer;
        let chosen_style: String = my_character.chose_style();
        dbg!("My character chose the style: {}", chosen_style);
    }
}

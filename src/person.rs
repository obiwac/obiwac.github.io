use maud::{html, Markup};

pub enum Person {
	Noa,
	Alexis,
	Alex,
	Drakeerv,
	Juk,
	Brichant,
	Aless,
	Piwy,
	Aditya,
}

pub fn person(person: Person) -> Markup {
	html! {
		@match person {
			Person::Noa => a.link href="https://novation.dev" { "Noa" },
			Person::Alexis => a.link href="https://github.com/alexisloic21" { "Alexis" },
			Person::Alex => a.link href="https://github.com/alleyezoncode" { "Alex" },
			Person::Drakeerv => a.link href="https://github.com/drakeerv" { "@drakeerv" },
			Person::Juk => a.link href="https://github.com/jukitsu" { "@jukitsu" },
			Person::Brichant => a.link href="http://brichant.eu" { "Monsieur Brichant" },
			Person::Aless => a.link href="https://github.com/akialess" { "Aless" },
			Person::Piwy => a.link href="https://github.com/Piwy-dev" { "Piwy" },
			Person::Aditya => a.link href="https://adityachugh.be" { "Aditya" },
		}
	}
}

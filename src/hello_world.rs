use bevy::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(persons_with_jobs)
            .add_system(persons_ready_for_hire)
            .add_system(person_does_job);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn((
        Person {
            name: "Charlie".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn(Person {
        name: "David".to_string(),
    });
    commands.spawn(Person {
        name: "Ellen".to_string(),
    });
}

fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "Fire Fighter",
            Job::Lawyer => "Lawyer",
        };
        println!("{} is a {}", person.name, job_name);
    }
}

fn persons_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for hire.", person.name);
    }
}

fn persons_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}

#[derive(Component)]
struct Person {
    pub name: String,
}

#[derive(Component)]
struct Employed {
    pub job: Job,
}

#[derive(Debug)]
enum Job {
    Doctor,
    Lawyer,
    FireFighter,
}

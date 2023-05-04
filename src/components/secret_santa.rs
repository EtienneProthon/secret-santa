use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaTrash, FaHeart, FaChevronRight, FaGift, FaCircleExclamation};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};

use crate::AppError;

const MAX_RETRY: u32 = 50;

// HashMap prevent people to give more than one gift
pub type Participants = HashSet<String>;
pub type Couples = HashMap<String, String>;
pub type GiftResult = HashMap<String, String>;

pub fn secret_santa(cx: Scope) -> Element {
    let error = use_state(cx, || None);
    let is_generating = use_state(cx, || false);
    let name = use_state(cx, || "".to_string());
    let participants = use_state(cx, Participants::default);
    let couple_name1 = use_state(cx, || "".to_string());
    let couple_name2 = use_state(cx, || "".to_string());
    let couples = use_state(cx, Couples::default);
    let gift_result = use_state(cx, GiftResult::default);
    cx.render(rsx!(section {
        div { class: "container mx-auto p-4 flex flex-col",
            h2 { class: "mb-6 text-3xl font-bold font-heading text-center",
                "My Secret Santa"
            }
            div { class: "my-2",
                label { class: "label",
                    "Add Participants"
                }
                input { class: "input w-full max-w-xs",
                    placeholder: "Name",
                    value: "{name}",
                    oninput: move |evt| name.set(evt.value.clone()),
                    onkeydown: move |evt| {
                                if evt.key() == Key::Enter  && !name.is_empty() {
                                    participants.make_mut().insert(name.get().clone());
                                    name.set("".to_string());
                                }
                    }
                }
            }
            participants.iter().map(|p_name| rsx!(
                div { 
                    key: "{p_name}", 
                    class: "alert bg-gray-700 shadow-lg p-2 pl-4 my-1",
                    div {
                        span {"{p_name}"}
                    }
                    div { class: "flex-none",
                        button { class: "btn btn-sm btn-ghost",
                            onclick: move |_| {
                                participants.make_mut().remove(p_name);
                            },
                            Icon {
                                fill: "white",
                                icon: FaTrash,
                            }
                        }
                    }
                }
            ))
            
            div { class: "my-2",
                label { class: "label",
                    "Add couples"
                }
                div {
                    select {
                        class: "select w-full max-w-xs",
                        value: "{couple_name1}",
                        oninput: move |evt| couple_name1.set(evt.value.clone()),
                        option {
                            disabled: true,
                            value: "",
                            label: "Select name"
                        }
                        participants.iter().filter(|name| name != &couple_name2.get()).map(|name| rsx!(
                            option { key: "{name}",
                                "{name}"
                            }
                        ))
                    }
                    select {
                        class: "select w-full max-w-xs ml-4",
                        value: "{couple_name2}",
                        oninput: move |evt| couple_name2.set(evt.value.clone()),
                        option {
                            disabled: true,
                            value: "",
                            label: "Select name"
                        }
                        participants.iter().filter(|name| name != &couple_name1.get()).map(|name| rsx!(
                            option { key: "{name}",
                                "{name}"
                            }
                        ))
                    }
                    button { class: "btn btn-accent ml-4",
                        onclick: move |_| {
                            if !couple_name1.is_empty() && !couple_name2.is_empty() && !couples.get().contains_key(couple_name2.get()) {
                                couples.make_mut().insert(couple_name1.get().clone(), couple_name2.get().clone());
                            }
                        },
                        "Add"
                    }
                }
            }
            couples.iter().map(|(c_name1, c_name2)| rsx!(
                div { 
                    key: "{c_name1}-{c_name2}", 
                    class: "alert bg-gray-700 shadow-lg p-2 pl-4 my-1",
                    div {
                        span {"{c_name1}"}
                        Icon {
                            fill: "white",
                            icon: FaHeart,
                        }
                        span {"{c_name2}"}
                    }
                    div { class: "flex-none",
                        button { class: "btn btn-sm btn-ghost",
                            onclick: move |_| {
                                couples.make_mut().remove(c_name1);
                            },
                            Icon {
                                fill: "white",
                                icon: FaTrash,
                            }
                        }
                    }
                }
            ))
            if *is_generating.get() {
                rsx!{
                    progress {
                        class: "progress w-56"
                    }
                }
            } else {
                rsx!{
                    button { class: "btn btn-primary mt-8 mb-4",
                        onclick: move |_| {
                            is_generating.set(true);
                            error.set(None);
                            gift_result.set(GiftResult::default());
                            match attribute_gift(participants.get(), couples.get(), 0) {
                                Ok(res) => gift_result.set(res),
                                Err(err) => error.set(Some(err))
                            }
                            is_generating.set(false);
                        },
                        "Generate !"
                    }

                }
            }
            if let Some(err) = error.get() {
                rsx!{div { 
                    class: "alert alert-error shadow-lg pl-4 my-1 justify-center text-white",
                    div {
                        Icon {
                            fill: "white",
                            icon: FaCircleExclamation,
                        }
                        "{err}"
                    }
                }}
            }
            gift_result.iter().map(|(giver, receiver)| rsx!(
                div { 
                    key: "{giver}-{receiver}", 
                    class: "alert alert-success shadow-lg pl-4 my-1 justify-center text-white",
                    div {
                        span {"{giver}"}
                        Icon {
                            fill: "white",
                            icon: FaChevronRight,
                        }
                        Icon {
                            fill: "white",
                            icon: FaGift,
                        }
                        Icon {
                            fill: "white",
                            icon: FaChevronRight,
                        }
                        span {"{receiver}"}
                    }
                }
            ))
        }
    }))
}

fn attribute_gift(participants: &Participants, couples: &Couples, retry_nb: u32) -> Result<GiftResult, AppError> {
    let mut gift_result = GiftResult::default();
    let mut blacklist: HashSet<(&str, Option<&str>)> = HashSet::new();

    let mut people = Vec::from_iter(participants.iter());

    // Exclude couples from being matched
    for (a, b) in couples {
        blacklist.insert((a, Some(b)));
        blacklist.insert((b, Some(a)));
    }

    // Shuffle people
    let mut rng = rand::thread_rng();
    people.shuffle(&mut rng);

    // Match people
    for person in &people {
        // Find who will offer a gift to this person

        // Check that a match can be found for this person
        let possible_matches: Vec<&String> = people
            .iter()
            .filter(|&&other| {
                &other != person
                    && !blacklist.contains(&(other, None))
                    && !blacklist.contains(&(person, Some(other)))
            })
            .copied()
            .collect();

        if possible_matches.is_empty() {
            // If no match is possible, start over
            let retry_nb = retry_nb + 1;
            if retry_nb >= MAX_RETRY {
                return Err(AppError::AttemptsLimitReached)
            }
            return attribute_gift(participants, couples, retry_nb);
        }

        // Choose a random match
        let match_index = rng.gen_range(0..possible_matches.len());
        let match_person = possible_matches[match_index];

        // Assign the match
        gift_result.insert(person.to_string(), match_person.to_string());

        // Prevent reciprocity
        blacklist.insert((match_person, Some(person)));
        blacklist.insert((match_person, None));
    }

    Ok(gift_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_attribute_gift_in_good_conditions() {
        let participants = HashSet::from([
            "Florent".to_string(),
            "Jessica".to_string(),
            "Coline".to_string(),
            "Emilien".to_string(),
            "Ambroise".to_string(),
            "Bastien".to_string(),
        ]);
        let couples = HashMap::from([
            ("Florent".to_string(), "Jessica".to_string()),
            ("Coline".to_string(), "Emilien".to_string()),
        ]);
        let res = attribute_gift(&participants, &couples, 0).unwrap();

        // Check couples don't give to each other
        for (name1, name2) in couples {
            assert_ne!(res.get(&name1).unwrap(), &name2);
            assert_ne!(res.get(&name2).unwrap(), &name1);
        }

        // Check nobody offer gift to himself
        for (giver, receiver) in &res {
            // println!("{giver} --> {receiver}");
            assert_ne!(giver, receiver)
        }

        // Check nobody get more than one gift
        for participant in &participants {
            assert_eq!(
                res.values()
                    .into_iter()
                    .filter(|x| x == &participant)
                    .count(),
                1
            )
        }
    }

    #[test]
    fn fail_in_bad_conditions() {
        let participants = HashSet::from([
            "Florent".to_string(),
            "Jessica".to_string(),
            "Emilien".to_string(),
        ]);
        let couples = HashMap::from([
            ("Florent".to_string(), "Jessica".to_string()),
        ]);
        let res = attribute_gift(&participants, &couples, 0);
        assert_eq!(res, Err(AppError::AttemptsLimitReached))

    }
}

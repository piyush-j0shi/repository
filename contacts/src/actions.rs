use crate::model::{Contact, ContactType};
use crate::utils::read_input;

pub fn add_contact(contacts: &mut Vec<Contact>) {
    let name = read_input("enter your name: ");

    if contacts.iter().any(|c| c.name == name) {
        println!("contact already exists");
        return;
    }

    let contact = read_input("enter the contact no: ");

    println!("available contact types: Personal, Work, Other");
    let contact_type = read_input("enter the contact type: ");

    let contact_typ = match contact_type.as_str() {
        "Personal" => ContactType::Personal,
        "Work" => ContactType::Work,
        "Other" => ContactType::Other,
        _ => {
            println!("invalid input, setting contact type as `Other`");
            ContactType::Other
        }
    };

    contacts.push(Contact {
        name,
        phone: contact,
        group: contact_typ,
    });

    println!("contact added");
}

pub fn view_contacts(contacts: &Vec<Contact>) {
    if contacts.is_empty() {
        println!("no contacts found");
        return;
    }

    for (i, contact) in contacts.iter().enumerate() {
        println!(
            "item no: {}, name: {}, contact number: {}, contact type: {:?}",
            i + 1,
            contact.name,
            contact.phone,
            contact.group
        );
    }
}

pub fn search_contact(contacts: &Vec<Contact>) {
    if contacts.is_empty() {
        println!("no contacts available");
        return;
    }

    let searched = read_input("enter the name to be searched: ");
    let mut found = false;

    for contact in contacts {
        if contact.name == searched {
            println!("contact found:");
            println!(
                "name: {}, contact number: {}, contact type: {:?}",
                contact.name, contact.phone, contact.group
            );
            found = true;
        }
    }

    if !found {
        println!("no contact found");
    }
}

pub fn edit_name(contacts: &mut Vec<Contact>) {
    view_contacts(contacts);
    if contacts.is_empty() {
        println!("no contacts available");
        return;
    }

    let previous_name = read_input("enter the name you want to change: ");
    let new_name = read_input("enter new name: ");

    let mut found = false;

    for contact in contacts.iter_mut() {
        if contact.name == previous_name {
            contact.name = new_name.clone();
            found = true;
            break;
        }
    }

    if found {
        println!("name changed successfully");
    } else {
        println!("no contact with that name found");
    }
}

pub fn edit_phone(contacts: &mut Vec<Contact>) {
    view_contacts(contacts);
    if contacts.is_empty() {
        println!("no contacts available");
        return;
    }

    let name = read_input("enter the name of the contact whose number you want to change: ");
    let new_number = read_input("enter the new phone number: ");

    if contacts.iter().any(|c| c.phone == new_number) {
        println!("this phone number is already used by another contact.");
        return;
    }

    if let Some(contact) = contacts.iter_mut().find(|c| c.name == name) {
        contact.phone = new_number;
        println!("phone number updated successfully");
    } else {
        println!("no contact with that name found");
    }
}

pub fn delete_contact(contacts: &mut Vec<Contact>) {
    view_contacts(contacts);
    if contacts.is_empty() {
        println!("no contacts available");
        return;
    }

    let input = read_input("enter the contact's item no to be deleted: ");
    if let Ok(index) = input.parse::<usize>() {
        if index == 0 || index > contacts.len() {
            println!("invalid contact number");
            return;
        }
        contacts.remove(index - 1);
        println!("contact deleted");
    } else {
        println!("invalid input");
    }
}

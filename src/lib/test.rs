// SPDX-FileCopyrightText: 2021 - 2022 Samuel W. Flint <swflint@flintfam.org>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub fn rename_repository(config: &mut Config, name: &String, newName: &String) {
    let mut repo = config.repositories.get_mut(&name.to_string());
    match repo {
        Some(repo) => {
            repo.name = newName.to_string();
            config.repositories.insert(newName.to_string(), repo);
            config.repositories.remove(&name.to_string());
        },
        None => panic!("No known repostory named \"{}\".", name)
    }
}

//! Codegen script for the NZFCC crate, update the downloaded codes by running:
//!
//! ```bash
//!     wget https://nzfcc.org/downloads/categories.json
//! ```

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Category {
    #[serde(rename = "_id")]
    id: String,
    name: String,
    groups: Groups,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Groups {
    personal_finance: Group,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Group {
    #[serde(rename = "_id")]
    id: String,
    name: String,
}

fn construct_group_enum(groups: &[Group]) -> String {
    let mut enum_def = String::from("/// An enum of the possible NZFCC category groups.\n");
    enum_def.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    enum_def.push_str(
        "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n",
    );
    enum_def.push_str("#[cfg_attr(feature = \"serde\", serde(deny_unknown_fields))]\n");
    enum_def.push_str("#[non_exhaustive]\n");
    enum_def.push_str("pub enum CategoryGroup {\n");
    for Group { name, .. } in groups {
        let enum_name = name
            .chars()
            .filter(|c| c.is_alphanumeric())
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        enum_def.push_str(&format!("    /// The \"{}\" group.\n", name));
        enum_def.push_str(&format!(
            "    #[cfg_attr(feature = \"serde\", serde(rename = \"{}\"))]\n",
            name
        ));
        enum_def.push_str(&format!("    {},\n", enum_name));
    }
    enum_def.push_str("}\n");

    // Add a conversion from CategoryGroup to Id
    enum_def.push_str("\n");
    enum_def.push_str("impl CategoryGroup {\n");
    enum_def.push_str("    /// Returns the ID of the category group.\n");
    enum_def.push_str("    /// Category group ID strings are always prefixed by `group_`.\n");
    enum_def.push_str("    pub const fn id(&self) -> &'static str {\n");
    enum_def.push_str("        match self {\n");
    for Group { name, id } in groups {
        let enum_name = name
            .chars()
            .filter(|c| c.is_alphanumeric())
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        enum_def.push_str(&format!("            Self::{} => \"{}\",\n", enum_name, id));
    }
    enum_def.push_str("        }\n");
    enum_def.push_str("    }\n");
    enum_def.push_str("}\n");

    enum_def
}

fn construct_nzfcc_code_enum(categories: &[Category]) -> String {
    let mut enum_def = String::new();
    enum_def.push_str("/// All possible New Zealand Financial Category Codes (NZFCC) codes, as defined by the NZFCC org [https://nzfcc.org/explore/](https://nzfcc.org/explore/).\n");
    enum_def.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    enum_def.push_str(
        "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n",
    );
    enum_def.push_str("#[cfg_attr(feature = \"serde\", serde(deny_unknown_fields))]\n");
    enum_def.push_str("#[non_exhaustive]\n");
    enum_def.push_str("pub enum NzfccCode {\n");
    for Category { name, .. } in categories {
        let enum_name = name
            .split_whitespace()
            .flat_map(|word| {
                let mut chars = word.chars();
                let first = chars.next().map(|c| c.to_ascii_uppercase());
                first.into_iter().chain(chars)
            })
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();

        enum_def.push_str(&format!("    /// The \"{}\" category.\n", name));
        enum_def.push_str(&format!(
            "    #[cfg_attr(feature = \"serde\", serde(rename = \"{}\"))]\n",
            name
        ));
        enum_def.push_str(&format!("    {},\n", enum_name));
    }
    enum_def.push_str("}\n");

    // Add a conversion from NzfccCode to Id
    enum_def.push_str("\n");
    enum_def.push_str("impl NzfccCode {\n");
    enum_def.push_str("    /// Returns the ID of the NZFCC code, NZFCC ids start with `nzfcc_`.\n");
    enum_def.push_str("    pub const fn id(&self) -> &'static str {\n");
    enum_def.push_str("        match self {\n");
    for Category { name, id, .. } in categories {
        let enum_name = name
            .split_whitespace()
            .flat_map(|word| {
                let mut chars = word.chars();
                let first = chars.next().map(|c| c.to_ascii_uppercase());
                first.into_iter().chain(chars)
            })
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        enum_def.push_str(&format!("            Self::{} => \"{}\",\n", enum_name, id));
    }
    enum_def.push_str("        }\n");
    enum_def.push_str("    }\n");
    enum_def.push_str("}\n");

    enum_def
}

fn main() {
    // Read the categories.json file in package root.
    let data = std::fs::read_to_string("categories.json").expect("Failed to read categories.json");
    let categories: Vec<Category> = serde_json::from_str(&data).expect("Failed to parse JSON");

    println!("cargo:rerun-if-changed=categories.json");

    let mut output_string = String::new();

    // Find all unique group names and create an enum.
    let mut seen_groups = HashSet::new();
    let filtered_groups = categories
        .iter()
        .map(|cat| &cat.groups.personal_finance)
        .filter(|group| seen_groups.insert(&group.id))
        .cloned()
        .collect::<Vec<Group>>();
    let group_enum = construct_group_enum(&filtered_groups);
    output_string.push_str(&group_enum);

    // Write to the output directory.
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("category_groups.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(output_string.as_bytes()).unwrap();

    // Find all the categories and create an enum.
    let nzfcc_enum = construct_nzfcc_code_enum(&categories);
    let dest_path = Path::new(&out_dir).join("nzfcc_codes.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(nzfcc_enum.as_bytes()).unwrap();
}

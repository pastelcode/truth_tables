use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Cell, Color, Table,
};
use std::{collections::HashMap, env, fmt::Display};

#[derive(Clone, Copy, Debug)]
struct TruthValue(bool);

impl Display for TruthValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { "V" } else { "F" })
    }
}

impl TruthValue {
    /// Creates a new [`TruthValue`].
    fn new() -> Self {
        TruthValue(true)
    }

    /// Toggles the value of this [`TruthValue`].
    fn toggle(&mut self) {
        self.0 = !self.0;
    }
}

fn main() {
    // |- Get propositions
    let mut propositions = env::args().collect::<Vec<String>>();
    // Removes the first element of arguments, the path of this executable.
    propositions.remove(0);
    propositions.dedup();
    // Get propositions -|

    let number_of_propositions: u32 = propositions.len().try_into().unwrap();
    let number_of_rows = 2_i32.pow(number_of_propositions);

    let mut columns = HashMap::<String, Vec<TruthValue>>::new();

    (0..)
        .zip(propositions.clone())
        .for_each(|(index, proposition)| {
            let group_size = 2_i32.pow(number_of_propositions - (index + 1));

            let mut values = Vec::<TruthValue>::new();
            let mut truth_value = TruthValue::new();
            for row_number in 1..=number_of_rows {
                values.push(truth_value);
                if row_number % group_size == 0 {
                    truth_value.toggle();
                }
            }
            columns.insert(proposition, values);

            if group_size == 1 {
                return;
            }
        });

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .apply_modifier(UTF8_ROUND_CORNERS);

    table.set_header(
        propositions
            .clone()
            .iter()
            .map(|proposition| Cell::new(proposition))
            .collect::<Vec<Cell>>(),
    );

    (0..number_of_rows).for_each(|row| {
        table.add_row(
            propositions
                .iter()
                .map(|key| {
                    let value = columns.get(key).unwrap()[row as usize];
                    Cell::new(value).fg(if value.0 { Color::Green } else { Color::Red })
                })
                .collect::<Vec<Cell>>(),
        );
    });

    println!("{}", table);
}

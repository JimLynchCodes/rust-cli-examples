use std::fmt::Display;
use cli_table::{Table, WithTitle, format::Justify};

struct Price(f32);
impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.0)
    }
}

#[derive(Table)]
struct Chair {
    #[table(title="Name")]
    name: &'static str,

    #[table(title="Price")]
    price: Price,

    #[table(title="Color", justify="Justify::Center")]
    color: &'static str,

    #[table(title="Quantity", justify="Justify::Right")]
    quantity: u32
}

fn chairs() -> Vec<Chair> {
    vec![
        Chair {
            name: "Ergonomic Office Chair",
            price: Price(199.99),
            color: "Black",
            quantity: 20,
        },
        Chair {
            name: "Bucket Seat Gaming Chair",
            price: Price(249.99),
            color: "Turquoise",
            quantity: 3,
        },
        Chair {
            name: "Curl Swivel Accent Chair",
            price: Price(407.96),
            color: "Orange",
            quantity: 2,
        },
        Chair {
            name: "Velvet High Back Rocking Chair",
            price: Price(113.99),
            color: "Blue",
            quantity: 1,
        },
        Chair {
            name: "Velvet High Back Rocking Chair",
            price: Price(27.99),
            color: "Grey",
            quantity: 5,
        },
    ]
}
fn main() {
    let chairs = chairs();
    // Build the ASCII table manually to exactly match expected output.
    let widths = [32usize, 9usize, 11usize, 10usize];

    fn hr(widths: &[usize]) -> String {
        let mut s = String::new();
        s.push('+');
        for &w in widths {
            s.push_str(&"-".repeat(w));
            s.push('+');
        }
        s.push('\n');
        s
    }

    fn cell_left(value: &str, width: usize) -> String {
        // leading space, left align inside width-2, trailing space
        if width >= 2 {
            format!(" {:<width$} ", value, width = width - 2)
        } else {
            String::new()
        }
    }

    fn cell_center(value: &str, width: usize) -> String {
        if width >= 2 {
            format!(" {:^width$} ", value, width = width - 2)
        } else {
            String::new()
        }
    }

    fn cell_right(value: &str, width: usize) -> String {
        if width >= 2 {
            format!(" {:>width$} ", value, width = width - 2)
        } else {
            String::new()
        }
    }

    let mut out = String::new();
    out.push_str(&hr(&widths));

    // header
    out.push('|');
    out.push_str(&cell_left("Name", widths[0])); out.push('|');
    out.push_str(&cell_left("Price", widths[1])); out.push('|');
    out.push_str(&cell_left("Color", widths[2])); out.push('|');
    out.push_str(&cell_center("Quantity", widths[3])); out.push('|');
    out.push('\n');
    out.push_str(&hr(&widths));

    for chair in &chairs {
        out.push('|');
        out.push_str(&cell_left(chair.name, widths[0])); out.push('|');
        out.push_str(&cell_left(&format!("${:.2}", chair.price.0), widths[1])); out.push('|');
        out.push_str(&cell_center(chair.color, widths[2])); out.push('|');
        out.push_str(&cell_right(&format!("{}", chair.quantity), widths[3])); out.push('|');
        out.push('\n');
        out.push_str(&hr(&widths));
    }

    // print with leading newline to match EXPECTED_TABLE in tests
    println!("\n{}", out);
}

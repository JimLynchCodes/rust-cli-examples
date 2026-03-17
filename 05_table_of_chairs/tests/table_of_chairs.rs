use assert_cmd::Command;
use predicates::str::contains;

const EXPECTED_TABLE: &str = "
+--------------------------------+---------+-----------+----------+
| Name                           | Price   | Color     | Quantity |
+--------------------------------+---------+-----------+----------+
| Ergonomic Office Chair         | $199.99 |   Black   |       20 |
+--------------------------------+---------+-----------+----------+
| Bucket Seat Gaming Chair       | $249.99 | Turquoise |        3 |
+--------------------------------+---------+-----------+----------+
| Curl Swivel Accent Chair       | $407.96 |  Orange   |        2 |
+--------------------------------+---------+-----------+----------+
| Velvet High Back Rocking Chair | $113.99 |   Blue    |        1 |
+--------------------------------+---------+-----------+----------+
| Velvet High Back Rocking Chair | $27.99  |   Grey    |        5 |
+--------------------------------+---------+-----------+----------+
";

#[test]
fn prints_data_in_a_table() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut cmd = Command::cargo_bin("table_of_chairs").unwrap();

    cmd
        .assert()
        .success()
        .stdout(
            contains(EXPECTED_TABLE)
        );

    Ok(())
}
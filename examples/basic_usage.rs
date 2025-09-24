fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jsonnet_code = r#"
        {
            person: {
                name: "Alice",
                age: 30,
                hobbies: ["reading", "hiking", "coding"],
            },
            message: "Hello, " + self.person.name + "!",
            active: true,
        }
    "#;

    // Evaluate to a JsonnetValue
    let evaluated_value = rs_jsonnet::evaluate(jsonnet_code)?;
    println!("Evaluated Value:\n{:#?}\n", evaluated_value);

    // Evaluate to a JSON string
    let json_output = rs_jsonnet::evaluate_to_json(jsonnet_code)?;
    println!("JSON Output:\n{}\n", json_output);

    // Conditionally evaluate to YAML if the feature is enabled
    #[cfg(feature = "yaml")]
    {
        let yaml_output = rs_jsonnet::evaluate_to_yaml(jsonnet_code)?;
        println!("YAML Output:\n{}", yaml_output);
    }

    Ok(())
}

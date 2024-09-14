use {
    rand::{seq::SliceRandom, Rng},
    std::{fs::File, io::Write},
};

#[test]
#[ignore]
fn gen_import() -> Result<(), std::io::Error> {
    // Create or truncate the file "import"
    let mut file = File::create("import")?;

    // Define the character set for user IDs
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .chars()
        .collect();

    let mut rng = rand::thread_rng();


    let _ = file.write(
        (1..=1_000_000)
            .map(|_| {
                // Generate a random month (01 to 12)
                let random_month = format!("{:02}", rng.gen_range(1..=12));
                // Generate a random day (01 to 28)
                let random_day = format!("{:02}", rng.gen_range(1..=28));
                // Generate a random user ID of length 16
                let user: String = (0..16).map(|_| *chars.choose(&mut rng).unwrap()).collect();

                format!("{} {}-{}", user, random_month, random_day)
            })
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    );



    Ok(())
}

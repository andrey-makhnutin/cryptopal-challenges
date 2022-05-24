static HOST: &str = "127.0.0.1:3030";
static MAX_EMAIL_LEN: usize = 128;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let block_size = guess_block_size()?;
    println!("Guessed block size as {}", block_size);

    let mut add_chars = block_size * 2 - 1;
    let mut attempts = 0;
    let my_account = loop {
        add_chars += 1;
        let account = get_account_for_email(add_chars)?;
        let repeating_block =
            match my_cryptopals_lib::block_cipher::get_repeating_ecb_block(&account) {
                None => continue,
                Some(block) => block,
            };
        let rep_block_index =
            get_block_index(&account, &repeating_block).expect("repeating block not found, wth");
        println!("Repeating block index is {}", rep_block_index);
        let block_boundary_add = add_chars - block_size * 2;
        let b_add = String::from_iter(vec!['B'; block_boundary_add]);
        let a_add = String::from_iter(vec!['A'; block_size]);
        let c_add = String::from_iter(vec!['C'; block_size - "@example.".len() - 2]); // 2 accounts for urlencoding :(
        let admin_email = format!("test{}{}{}@example.admin", b_add, a_add, c_add);
        let admin_account = get_account_for_custom_email(&admin_email)?;
        if admin_account
            [(rep_block_index * block_size)..(rep_block_index * block_size + block_size)]
            != repeating_block
        {
            println!("something shifted, trying again");
            add_chars = block_size * 2 - 1;
            attempts += 1;
            if attempts > 5 {
                panic!("Out of attemts...");
            }
            continue;
        }
        let admin_block_index = rep_block_index + 2;
        let mut admin_role = admin_account
            [(admin_block_index * block_size)..(admin_block_index * block_size + block_size)]
            .to_vec();
        let (encrypted_user_role, mut block_boundary_email_len, mut encrypted_padding) =
            get_encrypted_user_role()?;
        println!(
            "Encrypted user role is {}",
            my_cryptopals_lib::bytes_to_hex(&encrypted_user_role)
        );
        let a_add = String::from_iter(vec!['A'; block_size]);
        let my_email_len = "amx+@andrey.mx".len() + b_add.len() + block_size;
        if block_boundary_email_len < my_email_len {
            block_boundary_email_len += block_size;
        }
        let c_add = String::from_iter(vec!['C'; block_boundary_email_len - my_email_len + 4]);

        let my_email = format!("test{}{}{}@andrey.mx", b_add, a_add, c_add);
        let mut my_account = get_account_for_custom_email(&my_email)?;
        if my_account[(rep_block_index * block_size)..(rep_block_index * block_size + block_size)]
            != repeating_block
        {
            println!("something shifted 2, trying again");
            add_chars = block_size * 2 - 1;
            attempts += 1;
            if attempts > 5 {
                panic!("Out of attemts...");
            }
            continue;
        }
        if my_account[my_account.len() - block_size..my_account.len()] != encrypted_user_role {
            println!("something shifted 3, trying again");
            add_chars = block_size * 2 - 1;
            attempts += 1;
            if attempts > 5 {
                panic!("Out of attemts...");
            }
            continue;
        }
        my_account.truncate(my_account.len() - block_size);
        my_account.append(&mut admin_role);
        my_account.append(&mut encrypted_padding);
        println!(
            "Try this account: {}",
            my_cryptopals_lib::bytes_to_hex(&my_account)
        );
        break my_account;
    };
    println!("Checking role...");
    let body = check_role_with_account(&my_account)?;
    println!("{}", body);
    Ok(())
}

fn check_role_with_account(account: &[u8]) -> Result<String, MyError> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let res = client
        .get(format!("http://{}/role", HOST))
        .header(
            "Cookie",
            format!("account={}", my_cryptopals_lib::bytes_to_hex(&account)),
        )
        .send()?
        .error_for_status()?;
    Ok(res.text()?)
}

fn get_block_index(data: &[u8], block: &[u8]) -> Option<usize> {
    for i in (0..data.len()).step_by(block.len()) {
        if data[i..i + block.len()] == *block {
            return Some(i / block.len());
        }
    }
    None
}

fn guess_block_size() -> Result<usize, MyError> {
    let mut add_chars = 0;
    let mut prev_account_size = get_account_for_email(0)?.len();
    loop {
        add_chars += 1;
        let account_size = get_account_for_email(add_chars)?.len();
        if account_size == prev_account_size {
            continue;
        }
        let prev_account_size_again = get_account_for_email(add_chars - 1)?.len();
        if prev_account_size_again != prev_account_size {
            prev_account_size = prev_account_size_again;
        } else {
            return Ok(account_size - prev_account_size);
        }
    }
}

fn get_encrypted_user_role() -> Result<(Vec<u8>, usize, Vec<u8>), MyError> {
    let mut add_chars = 0;
    let mut prev_account_size = get_account_for_email(0)?.len();
    loop {
        add_chars += 1;
        let new_account = get_account_for_email(add_chars)?;
        let account_size = new_account.len();
        if account_size == prev_account_size {
            continue;
        }
        let account = get_account_for_email(add_chars + 4)?;
        let prev_account_size_again = get_account_for_email(add_chars - 1)?.len();
        if prev_account_size_again != prev_account_size {
            prev_account_size = prev_account_size_again;
        } else {
            let block_size = account_size - prev_account_size;
            let encrypted_padding = &new_account[new_account.len() - block_size..new_account.len()];
            return Ok((
                account[account.len() - block_size..account.len()].to_vec(),
                add_chars + "@example.com".len() + "test".len(),
                encrypted_padding.to_vec(),
            ));
        }
    }
}

fn get_account_for_email(add_chars: usize) -> Result<Vec<u8>, MyError> {
    let add = String::from_iter(vec!['A'; add_chars]);
    let email = format!("test{}@example.com", add);
    get_account_for_custom_email(&email)
}

fn get_account_for_custom_email(email: &str) -> Result<Vec<u8>, MyError> {
    if email.len() > MAX_EMAIL_LEN {
        return Err("reached max email length".into());
    }
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let res = client
        .post(format!("http://{}/account", HOST))
        .form(&[("email", email)])
        .send()?
        .error_for_status()?;
    for c in res.cookies() {
        if c.name() == "account" {
            return my_cryptopals_lib::parse_hex_string(c.value()).map_err(|e| e.into());
        }
    }
    Err("no account cookie found".into())
}

#[derive(Debug)]
enum MyError {
    OtherError(String),
    ReqwestError(String),
}

impl From<&str> for MyError {
    fn from(s: &str) -> Self {
        Self::OtherError(String::from(s))
    }
}

impl From<String> for MyError {
    fn from(s: String) -> Self {
        Self::OtherError(s)
    }
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(format!("{}", err))
    }
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

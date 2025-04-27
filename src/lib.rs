pub fn latency4(websites: Vec<&str>) -> Result<(), reqwest::Error> {
    let mut handles = vec![];
    for website in websites {
        let client = reqwest::blocking::Client::builder()
            .local_address(Some("0.0.0.0".parse().unwrap())).build()?;
        let website = website.to_string();
        handles.push(std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let response = client
                .get(&website)
                .header("Cache-Control", "no-store")
                .send();
            match response {
                Ok(resp) => {
                    let latency = start.elapsed();
                    let status_code = resp.status().as_u16();
                    Ok((website, latency, status_code))
                }
                Err(e) => Err(e),
            }
        }));
    }
    let mut results = vec![];
    for handle in handles {
        match handle.join().unwrap() {
            Ok(result) => results.push(result),
            Err(e) => return Err(e),
        }
    }
    results.sort_by(|a, b| a.1.cmp(&b.1));
    for (website, latency, status_code) in results {
        println!(
            "Site: {}, Status: {}, Latency: {:?}",
            website, status_code, latency
        );
    }
    Ok(())
}

pub fn latency6(websites: Vec<&str>) -> Result<(), reqwest::Error> {
    let mut handles = vec![];
    for website in websites {
        let client = reqwest::blocking::Client::builder()
            .local_address(Some("::".parse().unwrap())).build()?;
        let website = website.to_string();
        handles.push(std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let response = client
                .get(&website)
                .header("Cache-Control", "no-store")
                .send();
            match response {
                Ok(resp) => {
                    let latency = start.elapsed();
                    let status_code = resp.status().as_u16();
                    Ok((website, latency, status_code))
                }
                Err(e) => Err(e),
            }
        }));
    }
    let mut results = vec![];
    for handle in handles {
        match handle.join().unwrap() {
            Ok(result) => results.push(result),
            Err(e) => return Err(e),
        }
    }
    results.sort_by(|a, b| a.1.cmp(&b.1));
    for (website, latency, status_code) in results {
        println!(
            "Site: {}, Status: {}, Latency: {:?}",
            website, status_code, latency
        );
    }
    Ok(())
}
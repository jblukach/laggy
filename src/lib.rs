pub fn latency(websites: Vec<&str>) {
    let client = reqwest::blocking::Client::new();
    let mut handles = vec![];
    for website in websites {
        let client = client.clone();
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
                    (website, latency, status_code)
                }
                Err(_e) => (website, std::time::Duration::new(0, 0), 0),
            }
        }));
    }
    let mut results = vec![];
    for handle in handles {
        let result = handle.join().unwrap();
        results.push(result);
    }
    results.sort_by(|a, b| a.1.cmp(&b.1));
    for (website, latency, status_code) in results {
        println!(
            "Site: {}, Status: {}, Latency: {:?}",
            website, status_code, latency
        );
    }
}
pub fn latency4(websites: Vec<&str>) -> Result<(), reqwest::Error> {

    let mut handles = vec![];

    for website in websites {
        for _ in 0..5 {
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
    }

    let mut results = vec![];

    for handle in handles {
        match handle.join().unwrap() {
            Ok(result) => results.push(result),
            Err(e) => return Err(e),
        }
    }

    let mut website_latencies = std::collections::HashMap::new();

    for (website, latency, _status_code) in &results {
        let entry = website_latencies.entry(website).or_insert(vec![]);
        entry.push(*latency);
    }

    let mut avg_latencies: Vec<_> = website_latencies.iter().map(|(website, latencies)| {
        let avg_latency = latencies.iter().sum::<std::time::Duration>() / (latencies.len() as u32);
        (website, avg_latency)
    }).collect();

    avg_latencies.sort_by_key(|&(_, avg_latency)| avg_latency);

    for (website, _) in &avg_latencies {
        let latencies = &website_latencies[*website];
        let min_latency = latencies.iter().min().unwrap();
        let max_latency = latencies.iter().max().unwrap();
        let avg_latency = latencies.iter().sum::<std::time::Duration>() / (latencies.len() as u32);
        println!(
            "{{\"site\": \"{}\", \"min\": \"{:?}\", \"max\": \"{:?}\", \"avg\": \"{:?}\"}}",
            website, min_latency, max_latency, avg_latency
        );
    }

    Ok(())
}

pub fn latency6(websites: Vec<&str>) -> Result<(), reqwest::Error> {

    let mut handles = vec![];

    for website in websites {
        for _ in 0..5 {
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
    }

    let mut results = vec![];

    for handle in handles {
        match handle.join().unwrap() {
            Ok(result) => results.push(result),
            Err(e) => return Err(e),
        }
    }

    let mut website_latencies = std::collections::HashMap::new();

    for (website, latency, _status_code) in &results {
        let entry = website_latencies.entry(website).or_insert(vec![]);
        entry.push(*latency);
    }

    let mut avg_latencies: Vec<_> = website_latencies.iter().map(|(website, latencies)| {
        let avg_latency = latencies.iter().sum::<std::time::Duration>() / (latencies.len() as u32);
        (website, avg_latency)
    }).collect();

    avg_latencies.sort_by_key(|&(_, avg_latency)| avg_latency);

    for (website, _) in &avg_latencies {
        let latencies = &website_latencies[*website];
        let min_latency = latencies.iter().min().unwrap();
        let max_latency = latencies.iter().max().unwrap();
        let avg_latency = latencies.iter().sum::<std::time::Duration>() / (latencies.len() as u32);
        println!(
            "{{\"site\": \"{}\", \"min\": \"{:?}\", \"max\": \"{:?}\", \"avg\": \"{:?}\"}}",
            website, min_latency, max_latency, avg_latency
        );
    }

    Ok(())
}
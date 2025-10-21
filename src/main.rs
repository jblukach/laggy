fn main() {
    if let Ok(ip) = reqwest::blocking::get("https://whoami.4n6ir.com") {
        if let Ok(text) = ip.text() {
            let url = format!("https://geo.4n6ir.com/?{}", text);
            if let Ok(geo) = reqwest::blocking::get(&url) {
                if let Ok(geo_text) = geo.text() {
                    println!("\"whoami\": {}", geo_text);
                }
            }
            if text.contains(":") {
                let websites = vec![
                    "https://ipv6.afs1.lag.4n6ir.com",
                    "https://ipv6.ape1.lag.4n6ir.com",
                    "https://ipv6.ape2.lag.4n6ir.com",
                    "https://ipv6.apne1.lag.4n6ir.com",
                    "https://ipv6.apne2.lag.4n6ir.com",
                    "https://ipv6.apne3.lag.4n6ir.com",
                    "https://ipv6.aps1.lag.4n6ir.com",
                    "https://ipv6.aps2.lag.4n6ir.com",
                    "https://ipv6.apse1.lag.4n6ir.com",
                    "https://ipv6.apse2.lag.4n6ir.com",
                    "https://ipv6.apse3.lag.4n6ir.com",
                    "https://ipv6.apse4.lag.4n6ir.com",
                    "https://ipv6.apse5.lag.4n6ir.com",
                    "https://ipv6.apse6.lag.4n6ir.com",
                    "https://ipv6.apse7.lag.4n6ir.com",
                    "https://ipv6.cac1.lag.4n6ir.com",
                    "https://ipv6.caw1.lag.4n6ir.com",
                    "https://ipv6.euc1.lag.4n6ir.com",
                    "https://ipv6.euc2.lag.4n6ir.com",
                    "https://ipv6.eun1.lag.4n6ir.com",
                    "https://ipv6.eus1.lag.4n6ir.com",
                    "https://ipv6.eus2.lag.4n6ir.com",
                    "https://ipv6.euw1.lag.4n6ir.com",
                    "https://ipv6.euw2.lag.4n6ir.com",
                    "https://ipv6.euw3.lag.4n6ir.com",
                    "https://ipv6.ilc1.lag.4n6ir.com",
                    "https://ipv6.mec1.lag.4n6ir.com",
                    "https://ipv6.mes1.lag.4n6ir.com",
                    "https://ipv6.mxc1.lag.4n6ir.com",
                    "https://ipv6.sae1.lag.4n6ir.com",
                    "https://ipv6.use1.lag.4n6ir.com",
                    "https://ipv6.use2.lag.4n6ir.com",
                    "https://ipv6.usw1.lag.4n6ir.com",
                    "https://ipv6.usw2.lag.4n6ir.com",                
                ];
                if let Err(e) = laggy::latency6(websites) {
                    eprintln!("Error occurred: {}", e);
                }
            } else {
                let websites = vec![
                    "https://ipv4.afs1.lag.4n6ir.com",
                    "https://ipv4.ape1.lag.4n6ir.com",
                    "https://ipv4.ape2.lag.4n6ir.com",
                    "https://ipv4.apne1.lag.4n6ir.com",
                    "https://ipv4.apne2.lag.4n6ir.com",
                    "https://ipv4.apne3.lag.4n6ir.com",
                    "https://ipv4.aps1.lag.4n6ir.com",
                    "https://ipv4.aps2.lag.4n6ir.com",
                    "https://ipv4.apse1.lag.4n6ir.com",
                    "https://ipv4.apse2.lag.4n6ir.com",
                    "https://ipv4.apse3.lag.4n6ir.com",
                    "https://ipv4.apse4.lag.4n6ir.com",
                    "https://ipv4.apse5.lag.4n6ir.com",
                    "https://ipv4.apse6.lag.4n6ir.com",
                    "https://ipv4.apse7.lag.4n6ir.com",
                    "https://ipv4.cac1.lag.4n6ir.com",
                    "https://ipv4.caw1.lag.4n6ir.com",
                    "https://ipv4.euc1.lag.4n6ir.com",
                    "https://ipv4.euc2.lag.4n6ir.com",
                    "https://ipv4.eun1.lag.4n6ir.com",
                    "https://ipv4.eus1.lag.4n6ir.com",
                    "https://ipv4.eus2.lag.4n6ir.com",
                    "https://ipv4.euw1.lag.4n6ir.com",
                    "https://ipv4.euw2.lag.4n6ir.com",
                    "https://ipv4.euw3.lag.4n6ir.com",
                    "https://ipv4.ilc1.lag.4n6ir.com",
                    "https://ipv4.mec1.lag.4n6ir.com",
                    "https://ipv4.mes1.lag.4n6ir.com",
                    "https://ipv4.mxc1.lag.4n6ir.com",
                    "https://ipv4.sae1.lag.4n6ir.com",
                    "https://ipv4.use1.lag.4n6ir.com",
                    "https://ipv4.use2.lag.4n6ir.com",
                    "https://ipv4.usw1.lag.4n6ir.com",
                    "https://ipv4.usw2.lag.4n6ir.com",
                ];
                if let Err(e) = laggy::latency4(websites) {
                    eprintln!("Error occurred: {}", e);
                } 
            }
        }
    }
}
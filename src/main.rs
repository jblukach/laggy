fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        if args[1] == "lag" {
            if args[2] == "ipv4" {
                let websites = vec![
                    "https://ipv4.afs1.lag.4n6ir.com",
                    "https://ipv4.ape1.lag.4n6ir.com",
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
                laggy::latency(websites);
            } else if args[2] == "ipv6" {
                let websites = vec![
                    "https://ipv6.afs1.lag.4n6ir.com",
                    "https://ipv6.ape1.lag.4n6ir.com",
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
                laggy::latency(websites);
            } else {
                println!("Use: 'ipv4' or 'ipv6'");
            }
        } else if args[1] == "dev" {
            if args[2] == "ipv4" {
                let websites = vec![
                    "https://ipv4.afs1.dev.4n6ir.com",
                    "https://ipv4.ape1.dev.4n6ir.com",
                    "https://ipv4.apne1.dev.4n6ir.com",
                    "https://ipv4.apne2.dev.4n6ir.com",
                    "https://ipv4.apne3.dev.4n6ir.com",
                    "https://ipv4.aps1.dev.4n6ir.com",
                    "https://ipv4.aps2.dev.4n6ir.com",
                    "https://ipv4.apse1.dev.4n6ir.com",
                    "https://ipv4.apse2.dev.4n6ir.com",
                    "https://ipv4.apse3.dev.4n6ir.com",
                    "https://ipv4.apse4.dev.4n6ir.com",
                    "https://ipv4.apse5.dev.4n6ir.com",
                    "https://ipv4.apse7.dev.4n6ir.com",
                    "https://ipv4.cac1.dev.4n6ir.com",
                    "https://ipv4.caw1.dev.4n6ir.com",
                    "https://ipv4.euc1.dev.4n6ir.com",
                    "https://ipv4.euc2.dev.4n6ir.com",
                    "https://ipv4.eun1.dev.4n6ir.com",
                    "https://ipv4.eus1.dev.4n6ir.com",
                    "https://ipv4.eus2.dev.4n6ir.com",
                    "https://ipv4.euw1.dev.4n6ir.com",
                    "https://ipv4.euw2.dev.4n6ir.com",
                    "https://ipv4.euw3.dev.4n6ir.com",
                    "https://ipv4.ilc1.dev.4n6ir.com",
                    "https://ipv4.mec1.dev.4n6ir.com",
                    "https://ipv4.mes1.dev.4n6ir.com",
                    "https://ipv4.mxc1.dev.4n6ir.com",
                    "https://ipv4.sae1.dev.4n6ir.com",
                    "https://ipv4.use1.dev.4n6ir.com",
                    "https://ipv4.use2.dev.4n6ir.com",
                    "https://ipv4.usw1.dev.4n6ir.com",
                    "https://ipv4.usw2.dev.4n6ir.com",
                ];
                laggy::latency(websites);
            } else if args[2] == "ipv6" {
                let websites = vec![
                    "https://ipv6.afs1.dev.4n6ir.com",
                    "https://ipv6.ape1.dev.4n6ir.com",
                    "https://ipv6.apne1.dev.4n6ir.com",
                    "https://ipv6.apne2.dev.4n6ir.com",
                    "https://ipv6.apne3.dev.4n6ir.com",
                    "https://ipv6.aps1.dev.4n6ir.com",
                    "https://ipv6.aps2.dev.4n6ir.com",
                    "https://ipv6.apse1.dev.4n6ir.com",
                    "https://ipv6.apse2.dev.4n6ir.com",
                    "https://ipv6.apse3.dev.4n6ir.com",
                    "https://ipv6.apse4.dev.4n6ir.com",
                    "https://ipv6.apse5.dev.4n6ir.com",
                    "https://ipv6.apse7.dev.4n6ir.com",
                    "https://ipv6.cac1.dev.4n6ir.com",
                    "https://ipv6.caw1.dev.4n6ir.com",
                    "https://ipv6.euc1.dev.4n6ir.com",
                    "https://ipv6.euc2.dev.4n6ir.com",
                    "https://ipv6.eun1.dev.4n6ir.com",
                    "https://ipv6.eus1.dev.4n6ir.com",
                    "https://ipv6.eus2.dev.4n6ir.com",
                    "https://ipv6.euw1.dev.4n6ir.com",
                    "https://ipv6.euw2.dev.4n6ir.com",
                    "https://ipv6.euw3.dev.4n6ir.com",
                    "https://ipv6.ilc1.dev.4n6ir.com",
                    "https://ipv6.mec1.dev.4n6ir.com",
                    "https://ipv6.mes1.dev.4n6ir.com",
                    "https://ipv6.mxc1.dev.4n6ir.com",
                    "https://ipv6.sae1.dev.4n6ir.com",
                    "https://ipv6.use1.dev.4n6ir.com",
                    "https://ipv6.use2.dev.4n6ir.com",
                    "https://ipv6.usw1.dev.4n6ir.com",
                    "https://ipv6.usw2.dev.4n6ir.com",
                ];
                laggy::latency(websites);
            } else {
                println!("Use: 'ipv4' or 'ipv6'");
            }
        } else {
            println!("Use: 'lag' or 'dev'");
        }
    } else {
        println!("Use: 'lag' or 'dev' Next: 'ipv4' or 'ipv6'");
    }
}
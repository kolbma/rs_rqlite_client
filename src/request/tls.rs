//! `tls`

#[cfg(any(feature = "ureq_tls", feature = "ureq_webpki"))]
#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[test]
    fn website_github_test() {
        let agt_builder = ureq::AgentBuilder::new()
            .https_only(true)
            .timeout(Duration::from_secs(5));
        let agent = agt_builder.build();
        let r = agent.get("https://github.com").call();
        assert!(r.is_ok(), "{:?}", r.unwrap_err());
        assert!((200..300).contains(&r.unwrap().status()));
    }
}

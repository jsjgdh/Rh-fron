use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct ComplianceFramework {
    pub name: String,
    pub status: String,
    pub coverage: f64,
    pub workflows: Vec<String>,
}

#[component]
pub fn Compliance() -> Element {
    let frameworks = use_signal(|| vec![
        ComplianceFramework {
            name: "GDPR".to_string(),
            status: "Compliant".to_string(),
            coverage: 0.95,
            workflows: vec!["DataPurge".to_string(), "ConsentTracker".to_string()],
        },
    ]);

    rsx! {
        div { "Compliance for {frameworks.read().len()} frameworks" }
    }
}

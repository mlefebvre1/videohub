use super::{Label, OutputLock, OutputRoutings};
use std::fmt::Write;

#[derive(Default)]
pub struct Serializer {}

impl Serializer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize_video_output_routes(
        &self,
        output_routes: &OutputRoutings,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut s = "VIDEO OUTPUT ROUTING:\n".to_string();
        for route in output_routes.iter() {
            writeln!(s, "{} {}", route.destination - 1, route.source - 1)?;
        }
        s.push('\n');
        Ok(s)
    }

    pub fn serialize_output_labels(
        &self,
        labels: &[Label],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut s = "OUTPUT LABELS:\n".to_string();
        for label in labels {
            writeln!(s, "{} {}", label.id - 1, label.text)?;
        }
        s.push('\n');
        Ok(s)
    }

    pub fn serialize_input_labels(
        &self,
        labels: &[Label],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut s = "INPUT LABELS:\n".to_string();
        for label in labels.iter() {
            writeln!(s, "{} {}", label.id - 1, label.text)?;
        }
        s.push('\n');
        Ok(s)
    }

    pub fn serialize_output_locks(
        &self,
        output_locks: &[OutputLock],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut s = "VIDEO OUTPUT LOCKS:\n".to_string();
        for output_lock in output_locks.iter() {
            writeln!(s, "{} {}", output_lock.id - 1, output_lock.lock_status)?;
        }
        s.push('\n');
        Ok(s)
    }
}

#[test]
fn test_serialize_output_video_output_routes() {
    use super::Route;

    let routes = vec![Route {
        destination: 1,
        source: 2,
    }];
    let serializer = Serializer::new();
    assert_eq!(
        &serializer.serialize_video_output_routes(&routes).unwrap(),
        "VIDEO OUTPUT ROUTING:\n0 1\n\n"
    );

    let routes = vec![
        Route {
            destination: 1,
            source: 2,
        },
        Route {
            destination: 4,
            source: 3,
        },
    ];
    assert_eq!(
        &serializer.serialize_video_output_routes(&routes).unwrap(),
        "VIDEO OUTPUT ROUTING:\n0 1\n3 2\n\n",
    );
}

#[test]
fn test_serialize_output_labels() {
    let labels = vec![Label {
        id: 1,
        text: "Output Label 0".to_string(),
    }];
    // let labels = vec![(0_usize, "Output Label 0".to_string())];
    let expected = "OUTPUT LABELS:\n0 Output Label 0\n\n";
    let serializer = Serializer::new();
    assert_eq!(
        &serializer.serialize_output_labels(&labels).unwrap(),
        expected
    );

    let labels = vec![
        Label {
            id: 1,
            text: "Output Label 0".to_string(),
        },
        Label {
            id: 2,
            text: "Output Label 1".to_string(),
        },
    ];
    let expected = "OUTPUT LABELS:\n0 Output Label 0\n1 Output Label 1\n\n";
    let serializer = Serializer::new();
    assert_eq!(
        &serializer.serialize_output_labels(&labels).unwrap(),
        expected
    );
}

#[test]
fn test_serialize_input_labels() {
    let labels = vec![Label {
        id: 1,
        text: "Input Label 0".to_string(),
    }];
    let expected = "INPUT LABELS:\n0 Input Label 0\n\n";
    let serializer = Serializer::new();
    assert_eq!(
        &serializer.serialize_input_labels(&labels).unwrap(),
        expected
    );

    let labels = vec![
        Label {
            id: 1,
            text: "Input Label 0".to_string(),
        },
        Label {
            id: 2,
            text: "Input Label 1".to_string(),
        },
    ];
    let expected = "INPUT LABELS:\n0 Input Label 0\n1 Input Label 1\n\n";
    let serializer = Serializer::new();
    assert_eq!(
        &serializer.serialize_input_labels(&labels).unwrap(),
        expected
    );
}

#[test]
fn test_serialize_output_locks() {
    let lock_status = vec![
        OutputLock {
            id: 1,
            lock_status: super::LockStatus::Unlocked,
        },
        OutputLock {
            id: 2,
            lock_status: super::LockStatus::Locked,
        },
        OutputLock {
            id: 3,
            lock_status: super::LockStatus::ForceUnlock,
        },
        OutputLock {
            id: 4,
            lock_status: super::LockStatus::Owned,
        },
    ];
    let expected = "VIDEO OUTPUT LOCKS:\n0 U\n1 L\n2 F\n3 O\n\n";
    let serializer = Serializer::new();
    assert_eq!(
        &serializer.serialize_output_locks(&lock_status).unwrap(),
        expected
    );
}

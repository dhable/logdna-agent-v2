quick_error! {
    #[derive(Debug,Clone)]
    pub enum K8sError {
        PodMissingMetaError(descr: &'static str) {
            display("pod missing {}", descr)
        }
    }
}

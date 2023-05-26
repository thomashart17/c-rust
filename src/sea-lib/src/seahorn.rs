use bindings;

pub fn verifier_error() {
  unsafe { bindings::__VERIFIER_error(); }
}

pub fn assume(v: bool) {
  unsafe { bindings::__VERIFIER_assume(v.into()); }
}

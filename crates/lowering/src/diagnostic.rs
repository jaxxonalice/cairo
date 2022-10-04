use defs::diagnostic_utils::StableLocation;
use defs::ids::ModuleId;
use diagnostics::{DiagnosticEntry, DiagnosticLocation, Diagnostics, DiagnosticsBuilder};
use semantic::db::SemanticGroup;
use syntax::node::ids::SyntaxStablePtrId;
use syntax::node::TypedSyntaxNode;

pub struct LoweringDiagnostics {
    pub diagnostics: DiagnosticsBuilder<LoweringDiagnostic>,
    pub module_id: ModuleId,
}
impl LoweringDiagnostics {
    pub fn new(module_id: ModuleId) -> Self {
        Self { module_id, diagnostics: DiagnosticsBuilder::default() }
    }
    pub fn build(self) -> Diagnostics<LoweringDiagnostic> {
        self.diagnostics.build()
    }
    pub fn report<TNode: TypedSyntaxNode>(&mut self, node: &TNode, kind: LoweringDiagnosticKind) {
        self.diagnostics.add(LoweringDiagnostic {
            stable_location: StableLocation::from_ast(self.module_id, node),
            kind,
        });
    }
    pub fn report_by_ptr(&mut self, stable_ptr: SyntaxStablePtrId, kind: LoweringDiagnosticKind) {
        self.diagnostics.add(LoweringDiagnostic {
            stable_location: StableLocation::new(self.module_id, stable_ptr),
            kind,
        });
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LoweringDiagnostic {
    pub stable_location: StableLocation,
    pub kind: LoweringDiagnosticKind,
}
impl DiagnosticEntry for LoweringDiagnostic {
    type DbType = dyn SemanticGroup;

    fn format(&self, _db: &Self::DbType) -> String {
        match self.kind {}
    }

    fn location(&self, db: &Self::DbType) -> DiagnosticLocation {
        self.stable_location.diagnostic_location(db.upcast())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LoweringDiagnosticKind {}
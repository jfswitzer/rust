//! jfs - hooks for memoization

use crate::transform::{MirPass, MirSource};
use rustc_middle::mir::visit::MutVisitor;
use rustc_middle::mir::*;
use rustc_middle::ty::TyCtxt;

pub struct Memoize<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> Memoize<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Memoize { tcx }
    }
}

impl<'tcx> MirPass<'tcx> for Memoize<'tcx> {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, _: MirSource<'tcx>, body: &mut Body<'tcx>) {
        run_memoize(tcx, body);
    }
}

pub fn run_memoize<'tcx>(tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
    Memoize::new(tcx).visit_body(body);
}

impl<'tcx> MutVisitor<'tcx> for Memoize<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    fn visit_basic_block_data(&mut self, _block: BasicBlock, data: &mut BasicBlockData<'tcx>) {
	let term = data.terminator.clone();
	if term.is_some() {
	    let unwrapped = term.unwrap();
	    let kind = unwrapped.kind;
	    match kind {
		TerminatorKind::Call { func, args, .. } => {
		    debug!("Call to {:?} with args {:?} \n", func, args);
		}
		_ => {}
	    }
	}
    }
}

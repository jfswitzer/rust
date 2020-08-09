//! jfs - hooks for memoization

use std::fs::OpenOptions;
use crate::transform::{MirPass, MirSource};
use rustc_middle::mir::visit::MutVisitor;
use rustc_middle::mir::*;
use rustc_middle::ty::TyCtxt;
use std::io::Write;
use std::env;

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
	let key = "MEMO";
	let visit = match env::var(key) {
	    Ok(val) => { val == "yes".to_string() }
	    Err(_e) => false
	};
	if !visit {
	    return
	}
	let term = data.terminator.clone();
	if term.is_some() {
	    let unwrapped = term.unwrap();
	    let kind = unwrapped.kind;
	    match kind {
		TerminatorKind::Call { func, args, .. } => {
		    let file = OpenOptions::new().append(true).create(true).open("functions_i_found.csv");
		    match file {
			Ok(mut v) => {
			    let fstring = format!("{:?}", func).replace(",", "");
			    let astring = format!("{:?}", args).replace(",","");
			    v.write(b"\"").ok();
			    v.write(fstring.as_bytes()).ok();
			    v.write(b"\"").ok();			    
			    v.write(b",").ok();
			    v.write(b"\"").ok();			    
			    v.write(astring.as_bytes()).ok();
			    v.write(b"\"").ok();			    
			    v.write(b"\n").ok();
			},
			Err(_e) => {},
		    };
		}
		_ => {}
	    }
	}
    }
}

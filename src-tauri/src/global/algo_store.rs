#![allow(dead_code)]

use crate::graph::graph_ds::Graph;
use crate::state_err_enums::ap::APResult;
use crate::state_err_enums::diff_cen::DiffCenResult;
use crate::state_err_enums::mincut::MinCutResult;
use crate::state_err_enums::most_sim_timeline::MostSimTResult;
use crate::state_err_enums::pred_state::PredStateResult;

pub struct AlgoStore {
    pub aps: APResult,
    pub mincut: MinCutResult,
    pub diff_cent: DiffCenResult,
    pub most_sim_t: MostSimTResult,
    pub pred_state: PredStateResult,
}

impl AlgoStore {
    pub fn new() -> Self {
        AlgoStore {
            aps: APResult::Empty(true),
            mincut: MinCutResult::Empty(true),
            diff_cent: DiffCenResult::Empty(true),
            most_sim_t: MostSimTResult::Empty(true),
            pred_state: PredStateResult::Empty(true),
        }
    }

    pub fn get_aps(&self) -> &APResult {
        &self.aps
    }

    pub fn get_mincut(&self) -> &MinCutResult {
        &self.mincut
    }

    pub fn get_diff_cent(&self) -> &DiffCenResult {
        &self.diff_cent
    }

    pub fn get_most_sim_t(&self) -> &MostSimTResult {
        &self.most_sim_t
    }

    pub fn get_pred_state(&self) -> &PredStateResult {
        &self.pred_state
    }

    pub fn set_aps(&mut self, aps: APResult) {
        self.aps = aps;
    }

    pub fn set_mincut(&mut self, mincut: MinCutResult) {
        self.mincut = mincut;
    }

    pub fn set_diff_cent(&mut self, diff_cent: DiffCenResult) {
        self.diff_cent = diff_cent;
    }

    pub fn set_most_sim_t(&mut self, most_sim_t: Graph) {
        self.most_sim_t = MostSimTResult::Success(most_sim_t);
    }

    pub fn set_pred_state(&mut self, pred_state: Graph) {
        self.pred_state = PredStateResult::Success(pred_state);
    }
}

use std::collections::vec_deque::VecDeque;
use std::ops::Deref;
use crate::recognition::ParsedTube;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TubeState(pub VecDeque<u32>);
impl From<ParsedTube> for TubeState {
    fn from(parsed: ParsedTube) -> Self { Self (
        parsed.colors
    )}
}
impl Deref for TubeState {
    type Target = VecDeque<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<VecDeque<u32>> for TubeState {
    fn from(inner: VecDeque<u32>) -> Self { Self(inner) }
}
impl TubeState {
    pub fn is_solved(&self) -> bool {
        self.is_empty() || self.count_top_color() == 4
    }
    
    pub fn is_full(&self) -> bool {
        self.len() == 4
    }
    
    pub fn count_top_color(&self) -> usize {
        if self.is_empty() { return 0 }
        
        let color = *self.back().unwrap();
        let mut count = 1;
        
        for i in (0..(self.len() - 1)).rev() {
            if self[i] != color {
                break
            }
            
            count += 1;
        }
        
        count
    }
}


pub struct PuzzleStateNode {
    pub transfers: Vec<Transfer>,
    pub state: PuzzleState,
}


#[derive(Clone, Debug, Default, PartialEq)]
pub struct PuzzleState(pub Vec<TubeState>);
impl Deref for PuzzleState {
    type Target = Vec<TubeState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PuzzleState {
    pub fn solve(&self) -> Option<PuzzleStateNode> {
        let mut explored_states: Vec<PuzzleState> = vec![self.clone()];
        let mut queue = VecDeque::from([PuzzleStateNode { transfers: vec![], state: self.clone() }]);
        
        let mut solution_state = None;
        
        while !queue.is_empty() {
            match queue.pop_front() {
                Some(node) => {
                    if node.state.is_solved() {
                        solution_state = Some(node);
                        break;
                    }
                    
                    for (transfer, state) in node.state.possible_transfers() {
                        if !explored_states.contains(&state) {
                            explored_states.push(state.clone());
                            
                            let mut transfers = node.transfers.clone();
                            transfers.push(transfer);
                            queue.push_back(PuzzleStateNode {
                                transfers,
                                state,
                            })
                        }
                    }
                },
                None => break,
            }
        }
        
        solution_state
    }
    
    pub fn possible_transfers(&self) -> Vec<(Transfer, PuzzleState)> {
        let mut transfers = vec![];
        
        for from_i in 0..self.len() {
            for to_i in 0..self.len() {
                if from_i == to_i { continue }
                
                let transfer = Transfer { from: from_i, to: to_i };
                match transfer.try_perform(&self) {
                    Ok(state) => transfers.push((transfer, state)),
                    Err(_) => ()
                }
            }
        }
        
        transfers
    }
    
    pub fn is_solved(&self) -> bool {
        for tube in &self.0 {
            if !tube.is_solved() {
                return false;
            }
        }
        
        true
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Transfer {
    pub from: usize,
    pub to: usize,
}
impl Transfer {
    pub fn try_perform(&self, state: &PuzzleState) -> Result<PuzzleState, ()> {
        if self.from == self.to { return Err(()) }
        
        // The following code is safe because we've checked to make sure the 'to' and 'from' indices are different.
        // At this time, the compiler is not smart enough to recognize this.
        // `.split_at_mut()` is the "safe" alternative, but it needlessly complicates the code with extra checks and operations.
        // Using an implementation of UnsafeCell<T> could also work, but again, that is more code than what's used here.
        unsafe {
            let mut state = state.clone();
            let to: &mut TubeState = &mut *(state.0.get_unchecked_mut(self.to) as *mut _);
            if to.is_full() { return Err(()) }
            
            let from: &mut TubeState = &mut *(state.0.get_unchecked_mut(self.from) as *mut _);
            if from.is_empty() { return Err(()) }
            
            let from_count = from.count_top_color(); // mini-optimization
            
            if from_count == 4 { return Err(()) } // from tube is solved, no reason to move it
            
            if to.is_empty() {
                for _ in 0..from_count {
                    to.0.push_back(from.0.pop_back().unwrap());
                }
                return Ok(state);
            }
            
            if *from.back().unwrap() == *to.back().unwrap() {
                if from_count + to.len() <= 4 {
                    for _ in 0..from_count {
                        to.0.push_back(from.0.pop_back().unwrap());
                    }
                    return Ok(state);
                }
            }
            
            
            return Err(());
        }
    }
}

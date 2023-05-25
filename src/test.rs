macro_rules ! generate_id { ($ t : ty ; $ ($ name : ident) , + $ (,) ?) => { $ (# [doc = r" a wrapper for the id"] # [derive (Clone , Copy , PartialEq , PartialOrd , Ord , Eq , Hash)] # [repr (transparent)] pub struct $ name (pub $ t) ; impl $ name { pub fn new (id : $ t) -> Self { Self (id) } } impl std :: fmt :: Debug for $ name { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } } impl std :: fmt :: Display for $ name { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } } impl std :: ops :: Deref for $ name { type Target = $ t ; fn deref (& self) -> & Self :: Target { & self . 0 } } impl std :: ops :: DerefMut for $ name { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl std :: convert :: From < $ t > for $ name { fn from (id : $ t) -> Self { Self (id) } } impl std :: convert :: From < $ name > for $ t { fn from (id : $ name) -> Self { id . 0 } } impl Default for $ name { fn default () -> Self { Self (0) } }) + } ; }
generate_id ! (usize ; LogicRowId , LogicColId , PhysicRowId , PhysicColId , SubarrayId , RingId , RingBufferId , TsvId , WordId ,);
impl PhysicColId {
    pub fn word_id(&self) -> WordId {
        WordId(self.0 / 4)
    }
}
generate_id ! (u8 ; RingPort);
#[derive(Debug, Default, Clone)]
pub struct RowIdWordId {
    pub row_id: PhysicRowId,
    pub word_id: WordId,
}
impl RowIdWordId {
    pub fn new(row_id: PhysicRowId, word_id: WordId) -> Self {
        Self { row_id, word_id }
    }
}
#[derive(Debug, Default, Clone)]
pub struct RowLocation {
    pub subarray_id: SubarrayId,
    pub row_id_word_id: RowIdWordId,
}
impl RowLocation {
    pub fn new(subarray_id: SubarrayId, row_id_word_id: RowIdWordId) -> Self {
        Self {
            subarray_id,
            row_id_word_id,
        }
    }
}
pub trait JumpCycle {
    fn total(&self) -> usize;
    fn get_one_jump(&self) -> usize;
    fn get_multi_jump(&self) -> usize;
    fn get_one_jump_mut(&mut self) -> &mut usize;
    fn get_multi_jump_mut(&mut self) -> &mut usize;
}
pub trait AddableJumpCycle: JumpCycle {
    fn add(&mut self, jump_cycle: &Self);
}
pub trait UpdatableJumpCycle {
    fn update(
        &mut self,
        row_status: &RowIdWordId,
        loc: &RowLocation,
        size: WordId,
        remap_cycle: usize,
    );
}
pub trait RowCycleAction {
    fn apply<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(&mut self, item: &T);
}
pub trait RowCycleActionMut {
    fn apply_mut<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(&mut self, item: &mut T);
}
pub trait RowCycleActionPairMut {
    fn apply_pair_mut<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(
        &mut self,
        source: &T,
        target: &mut T,
    );
}
pub trait RowCycleArrayReduce {
    fn apply_reduce<T: JumpCycle + UpdatableJumpCycle + AddableJumpCycle>(
        &mut self,
        source: &[JumpCycles],
        target: &mut T,
        mapper: impl FnMut(&JumpCycles) -> &T,
    );
}
impl MyDuduAA {
    pub fn apply(&self, action: &mut impl RowCycleAction) {
        action.apply(&self.aaa);
        action.apply(&self.bbb);
        action.apply(&self.bb33);
        action.apply(&self.cc);
    }
    pub fn apply_mut(&mut self, action: &mut impl RowCycleActionMut) {
        action.apply_mut(&mut self.aaa);
        action.apply_mut(&mut self.bbb);
        action.apply_mut(&mut self.bb33);
        action.apply_mut(&mut self.cc);
    }
    pub fn apply_pair_mut(&self, target: &mut Self, action: &mut impl RowCycleActionPairMut) {
        action.apply_pair_mut(&self.aaa, &mut target.aaa);
        action.apply_pair_mut(&self.bbb, &mut target.bbb);
        action.apply_pair_mut(&self.bb33, &mut target.bb33);
        action.apply_pair_mut(&self.cc, &mut target.cc);
    }
    pub fn apply_reduce(
        input_array: &[Self],
        target: &mut Self,
        action: &mut impl RowCycleArrayReduce,
    ) {
        action.apply_reduce(input_array, &mut target.aaa, |item| &item.aaa);
        action.apply_reduce(input_array, &mut target.bbb, |item| &item.bbb);
        action.apply_reduce(input_array, &mut target.bb33, |item| &item.bb33);
        action.apply_reduce(input_array, &mut target.cc, |item| &item.cc);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MyDuduAATypes {
    Aaa,
    Bbb,
    Bb33,
    Cc,
    End,
}
impl Default for MyDuduAATypes {
    fn default() -> Self {
        Self::Aaa
    }
}
impl MyDuduAATypes {
    fn move_to_next(&mut self) {
        *self = match self {
            MyDuduAATypes::Aaa => MyDuduAATypes::Bbb,
            MyDuduAATypes::Bbb => MyDuduAATypes::Bb33,
            MyDuduAATypes::Bb33 => MyDuduAATypes::Cc,
            MyDuduAATypes::Cc => MyDuduAATypes::End,
            MyDuduAATypes::End => MyDuduAATypes::End,
        }
    }
}
impl Iterator for MyDuduAATypes {
    type Item = MyDuduAATypes;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.clone();
        if current == MyDuduAATypes::End {
            return None;
        }
        self.move_to_next();
        return Some(current);
    }
}

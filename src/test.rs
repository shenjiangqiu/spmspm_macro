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
#[derive(Debug, Default, Clone)]
pub struct RowLocation {
    pub subarray_id: SubarrayId,
    pub row_id_world_id: RowIdWordId,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Default, Clone, Copy)]
pub struct JumpCycles {
    pub arrow: Arrow,
    pub big_cack_goods: BigCackGoods,
    pub cack_bought_by_me_233_445: CackBoughtByMe<233, 445>,
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
impl JumpCycles {
    pub fn apply(&self, action: &mut impl RowCycleAction) {
        action.apply(&self.arrow);
        action.apply(&self.big_cack_goods);
        action.apply(&self.cack_bought_by_me_233_445);
    }
    pub fn apply_mut(&mut self, action: &mut impl RowCycleActionMut) {
        action.apply_mut(&mut self.arrow);
        action.apply_mut(&mut self.big_cack_goods);
        action.apply_mut(&mut self.cack_bought_by_me_233_445);
    }
    pub fn apply_pair_mut(&self, target: &mut Self, action: &mut impl RowCycleActionPairMut) {
        action.apply_pair_mut(&self.arrow, &mut target.arrow);
        action.apply_pair_mut(&self.big_cack_goods, &mut target.big_cack_goods);
        action.apply_pair_mut(
            &self.cack_bought_by_me_233_445,
            &mut target.cack_bought_by_me_233_445,
        );
    }
    pub fn apply_reduce(
        input_array: &[Self],
        target: &mut Self,
        action: &mut impl RowCycleArrayReduce,
    ) {
        action.apply_reduce(input_array, &mut target.arrow, |item| &item.arrow);
        action.apply_reduce(input_array, &mut target.big_cack_goods, |item| {
            &item.big_cack_goods
        });
        action.apply_reduce(input_array, &mut target.cack_bought_by_me_233_445, |item| {
            &item.cack_bought_by_me_233_445
        });
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JumpCyclesTypes {
    Arrow,
    BigCackGoods,
    CackBoughtByMe233G445,
    End,
}
impl JumpCyclesTypes {
    fn move_to_next(&mut self) {
        *self = match self {
            JumpCyclesTypes::Arrow => JumpCyclesTypes::BigCackGoods,
            JumpCyclesTypes::BigCackGoods => JumpCyclesTypes::CackBoughtByMe233G445,
            JumpCyclesTypes::CackBoughtByMe233G445 => JumpCyclesTypes::End,
            JumpCyclesTypes::End => JumpCyclesTypes::End,
        }
    }
}
impl Iterator for JumpCyclesTypes {
    type Item = JumpCyclesTypes;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.clone();
        if current == JumpCyclesTypes::End {
            return None;
        }
        self.move_to_next();
        return Some(current);
    }
}

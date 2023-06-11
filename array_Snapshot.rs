struct SnapshotArray {
    array: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut array = Vec::new();
        for _ in 0..length {
            array.push(vec![(0, 0)]);
        }
        SnapshotArray { array, snap_id: 0 }
    }

    fn set(&mut self, index: i32, val: i32) {
        let snapshots = &mut self.array[index as usize];
        snapshots.push((self.snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        self.snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snapshots = &self.array[index as usize];
        for i in (0..snapshots.len()).rev() {
            if snapshots[i].0 <= snap_id {
                return snapshots[i].1;
            }
        }
        0
    }
}
                                       
fn main() {
    let mut obj = SnapshotArray::new(1);
    obj.set(0, 4);
    obj.set(0, 16);
    obj.set(0, 13);
    println!("{}", obj.snap());  // Output: 0
    println!("{}", obj.get(0, 0));  // Output: 13
    println!("{}", obj.snap());  // Output: 1
}

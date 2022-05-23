use device_tree::DeviceTree;
use device_tree::Node;

pub unsafe fn init(dtb_pa: usize) {
    *NUM_HARTS.lock() = count_harts(dtb_pa)
}

lazy_static::lazy_static! {
    // 最大的硬件线程编号；只在启动时写入，跨核软中断发生时读取
    pub static ref NUM_HARTS: spin::Mutex<usize> = spin::Mutex::new(8);
}

#[repr(C)]
struct DtbHeader {
    magic: u32,
    size: u32,
}

unsafe fn count_harts(dtb_pa: usize) -> usize {
    let header = &*(dtb_pa as *const DtbHeader);
    // from_be 是大小端序的转换（from big endian）
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        // 拷贝数据，加载并遍历
        let data = core::slice::from_raw_parts(dtb_pa as *const u8, size as usize);
        if let Ok(dt) = DeviceTree::load(data) {
            if let Some(cpu_map) = dt.find("/cpus/cpu-map") {
                return enumerate_cpu_map(cpu_map);
            }
        }
    }
    // 如果DTB的结构不对（读不到/cpus/cpu-map），返回默认的8个核
    let ans = 8;
    ans
}

fn enumerate_cpu_map(cpu_map_node: &Node) -> usize {
    let mut tot = 0;
    for cluster_node in cpu_map_node.children.iter() {
        let name = &cluster_node.name;
        let count = cluster_node.children.iter().count();
        println!("[rustsbi-dtb] Hart count: {} with {} cores", name, count);
        tot += count;
    }
    tot
}

const DEVICE_TREE_MAGIC: u32 = 0xD00DFEED;
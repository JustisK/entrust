use std::collections::HashMap;

struct HNode {
    freq: i32,
    ch: Option<char>, 
    left: Option<Box<HNode>>,
    right: Option<Box<HNode>>,
}

impl HNode {

    pub fn new(freq: i32, ch: Option<char>) -> Self {
        HNode {
            freq: freq, ch: ch,
            left: None, right: None,
        }
    }

}

pub struct HuffmanCode {
    // The input distribution underlying a particular Huffman code
    // is provided via the `data` field, currently just the basis string.
    // TODO: provide a representation of the underlying _frequency map_
    // and change HuffmanCode::new() to take a freq_map rather than &str.
    data: String,  
    root: Box<HNode>,
    code: HashMap<char, String>
}

impl HuffmanCode {
    
    pub fn new(s: &str) -> Self {
        let root = generate_tree(s);
        let mut code: HashMap<char, String> = HashMap::new();
        assign_codes(&root, &mut code, "".to_string());
        
        HuffmanCode { data: s.to_string(), 
                      root: root, 
                      code: code 
        }
    }

    pub fn encode_string(&self, s: &str) -> String {
        
        let mut ret = "".to_string();
        let mut token: Option<&String>;
        
        for ch in s.chars() {
            token = self.code.get(&ch);
            ret.push_str(token.unwrap());
        }
        ret
    }

    pub fn decode_string(&self, s: &str) -> String {

        let mut ret = "".to_string();
        let mut node = &self.root;

        for x in s.chars() {
            if x == '0' { // walk left for 0
                if let Some(ref l) = node.left {
                    node = l;
                }
            } else {
                if let Some(ref r) = node.right {
                    node = r; // else (1), walk right
                }
            }
            if let Some(ch) = node.ch {
                ret.push(ch);
                node = &self.root;
            }
        }
        ret
    }

}

fn freq_map(s: &str) -> HashMap<char, i32> {
    let mut freq_map = HashMap::new();
    for ch in s.chars() {
        let count = freq_map.entry(ch).or_insert(0); // Get prior occurrences of character (initialize to 0 if none)
        *count += 1;
    }
    freq_map
}

fn generate_tree(s: &str) -> Box<HNode> {
    // Build frequency table
    let freq_map = freq_map(s);

    // Build nodelist
    let mut nodes: Vec<Box<HNode>> = 
            freq_map.iter()
              .map(|(k,v)| Box::new(HNode::new(*v, Some(*k))))
              .collect();

    // While there are nodes to merge...
    while nodes.len() > 1 {
        nodes.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        // pop off the smallest two nodes...
        let a = nodes.pop().unwrap();
        let b = nodes.pop().unwrap();
        // ...create a new node with those two as its children...
        let mut c = Box::new(HNode::new(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        // ...and reinsert the merged node at the other end of the queue.
        nodes.push(c);
    }
    nodes.pop().unwrap()
}

fn assign_codes(node: &Box<HNode>, // call this function with node == your root node
                codes: &mut HashMap<char, String>,
                code: String ){
    
    // If HNode has a valid 'ch' field, it's a leaf (base case)
    if let Some(ch) = node.ch {
        codes.insert(ch, code);
    } else { // walk the tree, appending l->0, r->1, until a leaf is reached
        if let Some(ref l) = node.left {
            assign_codes(l, codes, code.clone() + "0");
        }
        if let Some(ref r) = node.right {
            assign_codes(r, codes, code.clone() + "1");
        }
    }
}

#[cfg(test)]
mod test {
    
    use super::{HuffmanCode};
    use itertools::Itertools;

    #[test]
    fn test_compressor() {
        let strings = vec!["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                           "dagoth ur was a hotep", "fifty liquors yeah good",
                           "the quick brown fox jumped over the lazy dog"];
        for s in strings {
            attempt_compress(s);
        }
    }

    fn attempt_compress(s: &str) {
        let _s = s.clone();
        let encoder = HuffmanCode::new(_s);
        let bin_seq = encoder.encode_string(_s);
        let decoded_str = encoder.decode_string(&bin_seq.clone());
        // let nbits_in = 8*(_s.chars().count());
        // let nbits_out = bin_seq.chars().count();
        // let compression_ratio: f32 = nbits_out as f32 / nbits_in as f32;
        // println!("Compression ratio: {}", compression_ratio);
        assert_eq!(_s, decoded_str);
    }

    #[test]
    fn test_internals() {
        let encoder = HuffmanCode::new("dagoth ur was a hotep");
        let code1: Vec<&String> = encoder.code.values().collect();
        let code2: Vec<&String> = encoder.code.values().collect();
        assert_eq!(code1, code2);
        assert!(codewords_are_unique(code1));
        assert!(is_valid_prefix_code(code2));
    }
    
    fn codewords_are_unique(symbols: Vec<&String>) -> bool {
        // Ensure codewords are unique (no duplicates)
        for c in symbols.iter().combinations(2) {
            assert!(!((*c[0]) == (*c[1])));
        }
        true
    }

    fn is_valid_prefix_code(symbols: Vec<&String>) -> bool {
        // Ensure no codewords are prefixes of any other codewords (i.e., 'symbols' is a prefix code)
        for p in symbols.iter().permutations(2) {
            assert!(!((*p[0]).starts_with(*p[1])));
        }
        true
    }

}
use std::collections::HashSet;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum DigestAlgo {
    Md5,
    Sha1,
    Sha512   //nuget: Sha512 finalized with Base64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DigestExtTable {
    md5: HashSet<String>,
    sha1: HashSet<String>,
    sha512: HashSet<String>,
    blocked: HashSet<DigestAlgo>

}

impl DigestExtTable {
    pub fn is_blocked(&self, algo: DigestAlgo) -> bool {
        self.blocked.contains(&algo)
    }

    pub fn is_md5(&self, file_ext: String) -> bool {
        !self.is_blocked(DigestAlgo::Md5) && self.md5.contains(&file_ext)
    }

    pub fn is_sha1(&self, file_ext: String) -> bool {
        !self.is_blocked(DigestAlgo::Sha1) && self.sha1.contains(&file_ext)
    }

    pub fn is_sha512(&self, file_ext: String) -> bool {
        !self.is_blocked(DigestAlgo::Sha512) && self.sha512.contains(&file_ext)
    }

    pub fn add(&mut self, algo: DigestAlgo, file_ext: String) -> bool {
        match algo {
            DigestAlgo::Md5     => self.md5.insert(file_ext),
            DigestAlgo::Sha1    => self.sha1.insert(file_ext),
            DigestAlgo::Sha512  => self.sha512.insert(file_ext)
        }
    }

    pub fn clear(&mut self, algo: DigestAlgo) -> bool {
        match algo {
            DigestAlgo::Md5 => {
                self.md5.clear();
                self.md5.is_empty()
            },
            DigestAlgo::Sha1 => {
                self.sha1.clear();
                self.sha1.is_empty()
            },
            DigestAlgo::Sha512 => {
                self.sha512.clear();
                self.sha512.is_empty()
            }
        }
    }

    pub fn add_many(&mut self, algo: DigestAlgo, file_exts: Vec<String>) {

        for ext in &file_exts {
            self.add(algo, ext.to_string());
        }
    }

    pub fn block(&mut self, algo: DigestAlgo) -> bool {
        self.blocked.insert(algo)
    }

    pub fn swipe(&mut self) -> bool {
        self.blocked.clear();
        self.md5.clear();
        self.sha1.clear();
        self.sha512.clear();

        self.blocked.is_empty() && self.md5.is_empty() && self.sha1.is_empty()
    }
}

impl Default for DigestExtTable {

    fn default() -> DigestExtTable {
        let mut md5_exts = HashSet::new();
        md5_exts.insert("gz".to_string());
        md5_exts.insert("whl".to_string());

        let mut sha1_exts = HashSet::new();
        sha1_exts.insert("jar".to_string());
        sha1_exts.insert("tgz".to_string());

        let mut sha512_exts = HashSet::new();
        sha512_exts.insert("nupkg".to_string());

        DigestExtTable{
            md5: md5_exts,
            sha1: sha1_exts,
            sha512: sha512_exts,
            blocked: HashSet::new()
        }
    }
}

impl fmt::Debug for DigestExtTable {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        let mut blocked_algos = vec![];
        let mut file_exts = vec![];

        if self.is_blocked(DigestAlgo::Md5){
            blocked_algos.push("md5");
        } else {
            let exts = format!("md5: {:?}", self.md5);
            file_exts.push(exts);
        }

        if self.is_blocked(DigestAlgo::Sha1){
            blocked_algos.push("sha1");
        } else {
            let exts = format!("sha1: {:?}", self.sha1);
            file_exts.push(exts);
        }

        if self.is_blocked(DigestAlgo::Sha512){
            blocked_algos.push("sha512");
        } else {
            let exts = format!("sha512: {:?}", self.sha512);
            file_exts.push(exts);
        }

        if !blocked_algos.is_empty() {
            writeln!(f, "blocked algos: {}", blocked_algos.join(", ")).unwrap();
        }

        if !file_exts.is_empty() {
            writeln!(f, "File extensions:\n {}", file_exts.join("\n")).unwrap();
        }

        write!(f, "\n")
    }
}

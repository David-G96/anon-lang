use crate::buffered_iter::MultiBufferIter;
use crate::location::Location;

#[derive(Debug)]
pub struct LineMap {
    line_starts: Vec<usize>,
    len: usize,
}

impl LineMap {
    pub fn new(source: &str) -> Self {
        let len = source.len();
        let mut line_starts = vec![0];

        let char_indices = source.char_indices();
        let mut buffered_iter = MultiBufferIter::new(char_indices);

        loop {
            if let Some((idx, c)) = buffered_iter.next() {
                // 找到换行符 LF
                if c == '\n' {
                    // 换行符之后还有字符，说明下一个字符是行首
                    if let Some((next_idx, next_c)) = buffered_iter.next() {
                        buffered_iter.put_back((next_idx, next_c));
                        line_starts.push(next_idx);
                    }
                    // 如果换行符之后没有字符也不用管了，字符流就结束了
                    else {
                        break;
                    }
                }
                // 可能是换行符 CRLF
                else if c == '\r' {
                    //　之后是\n
                    if let Some((next_idx, '\n')) = buffered_iter.next() {
                        // CRLF之后还有字符，说明下一个字符是行首
                        if let Some((next_next_idx, next_next_c)) = buffered_iter.next()
                        {
                            buffered_iter.put_back((next_next_idx, next_next_c));
                            line_starts.push(next_next_idx);
                        } else {
                            break;
                        }
                    }
                    // \r后面不是\n或者没有后续字符，\r单独作为换行符处理
                    else {
                        // \r之后的字符是行首
                        if let Some((next_idx, next_c)) = buffered_iter.next() {
                            buffered_iter.put_back((next_idx, next_c));
                            line_starts.push(next_idx);
                        } else {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }

        Self { line_starts, len }
    }

    pub fn index_to_location(&self, idx: crate::span::SpanIndex) -> Option<Location> {
        let idx = idx as usize;
        if idx >= self.len {
            return None;
        }
        match self.line_starts.binary_search(&idx) {
            Ok(line_idx) => Some(Location::new(line_idx as u32, 0u32)),
            Err(line_idx) => {
                let line = line_idx - 1;
                let column = idx - self.line_starts[line];
                Some(Location::new(line as u32, column as u32))
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

// リングバッファの実装
// 
// 動作に少しクセがあり，例えばnew()で作ったリングバッファは
// 空のバッファではなく0で初期化された要素数1のバッファとなる．
// shift_l関数を使っても要素数を0にすることは出来ない．

pub const RING_SIZE: usize = 1024;

// Data range is [start, end)
// startとendが符号なし整数なので実装に注意
// 有効データがバッファサイズから溢れたらそのまま上書き
pub struct RingBuffer {
    buf: [u8; RING_SIZE],
    start: usize,  // 有効データの始点（閉区間）
    end:   usize,  // 　　〃　　　終点（開区間）
}

impl RingBuffer {
    pub fn new() -> RingBuffer {
        RingBuffer {
            buf: [0; RING_SIZE],
            start: 0,
            end:   1,
        }
    }

    /// 有効データ長
    pub fn len(&self) -> usize {
        let start = self.start;
        let end = self.end;

        if start < end {
            end - start
        } else {
            RING_SIZE - (start - end)
        }
    }

    /// 有効データ長さを1にしてゼロクリアする．
    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 1;
        self.buf[0] = 0;
    }

    /// valを有効データの末尾に追加
    /// return: overflow flag
    pub fn push(&mut self, val: u8) -> bool {
        let start = self.start;
        let end = self.end;
        let flag: bool;

        self.buf[end] = val;
        if start == end {
            self.start = (start + 1) % RING_SIZE;
            flag = true;
        } else {
            flag = false;
        }
        self.end = (end + 1) % RING_SIZE;
        flag
    }

    /// vecを有効データの末尾に追加
    /// return: overflow flag
    pub fn append(&mut self, vec: &mut Vec<u8>) -> bool {
        let mut flag = false;
        for i in 0..vec.len() {
            flag = self.push( vec[i] );
        }
        flag
    }

    /// 全有効データを取得
    pub fn get_all(&self) -> Vec<u8> {
        let data_len = self.len();
        let mut data = Vec::with_capacity(data_len);
        for i in 0..data_len {
            data.push( self.buf[ (self.start + i) % RING_SIZE ] );
        }
        data
    }

    /// 有効データの始点からindex番目のデータを読む
    /// 有効データの範囲外にアクセスしたらNoneを返す
    pub fn read(&self, index: usize) -> Option<u8> {
        if index < self.len() {
            let i = (index + self.start) % RING_SIZE;
            Some( self.buf[i] )
        } else {
            None
        }
    }

    /// 有効データ左シフト
    /// シフトした分だけ先頭のデータが消える．
    pub fn shift_l(&mut self, num: usize) -> Result<(), &'static str> {
        if num < self.len() {
            self.start = (self.start + num) % RING_SIZE;
        } else {
            return Err("Shift num is out of length.");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // テストはRING_SIZE=10としてから実行
    #[test]
    fn test_push() {
        let mut buf = RingBuffer::new();
        for i in 0..5 {
            buf.push(i);
        }
        assert_eq!(vec![0, 0, 1, 2, 3, 4], buf.get_all());
        assert_eq!(6, buf.len());
    }

    #[test]
    fn test_push_overflow() {
        let mut buf = RingBuffer::new();
        let mut flag = false;
        for i in 0..12 {
            flag = buf.push(i);
        }
        assert_eq!(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11], buf.get_all());
        assert_eq!(true, flag);
        assert_eq!(10, buf.len());

        let get_data = buf.get_all();
        let true_data = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(get_data.len(), true_data.len());
        assert_eq!(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11], get_data);
    }

    #[test]
    fn test_append() {
        let mut buf = RingBuffer::new();

        buf.append(&mut vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(buf.len(), 7);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6], buf.get_all());
    }

    #[test]
    fn test_shift_l() {
        let mut buf = RingBuffer::new();
        for i in 0..12 {
            buf.push(i);
        }
        buf.shift_l(4).unwrap();
        assert_eq!(vec![6, 7, 8, 9, 10, 11], buf.get_all());
        assert!( buf.shift_l(6).is_err() );
    }
}

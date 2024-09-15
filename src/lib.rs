pub struct FiloQueue<T> where T: Sized  {
    buf: Vec<T>,
    max: usize
}

impl<T> FiloQueue<T> {
    pub fn new(max: usize) -> FiloQueue<T> {
        return FiloQueue {
            buf: vec![], 
            max
        }
    }

    pub fn push(&mut self, value: T) {
        if self.buf.len() == self.max {
            self.pop();
        }

        self.buf.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.buf.is_empty() {
            return None;
        }

        return Some(self.buf.remove(0));
    }

    pub fn len(&self) -> usize {
        return self.buf.len();
    }

    pub fn as_slice(&self) -> &[T] {
        return self.buf.as_slice();
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

}



#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}

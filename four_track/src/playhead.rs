#[derive(Debug, Clone)]
enum Direction {
    Forward,
    Backward
}

#[derive(Debug, Clone)]
pub struct Playhead {
    // invariant: len(left_channel) == len(right_channel)
    /// samples for left channel
    left_channel: Vec<f32>,
    /// samples for right channel
    right_channel: Vec<f32>,
    
    /// current position within audio
    head_pos: usize,
    /// position of loop IN marker
    loop_in: usize,
    /// position of loop OUT marker
    loop_out: usize,
    /// is looping enabled
    looping: bool,
}

impl Playhead {
    pub const ONE_MINUTE_SECONDS: usize = 60;
    //pub const ONE_MINUTE_MILLIS: usize = 60000;

    /// create a Playhead instance, channels are zero length at this point
    pub fn new(sample_rate: usize, length_ms: usize) -> Self {
        let size = sample_rate * length_ms;
        Self {
            //left_channel: Vec::new(), 
            left_channel: vec![0.;size],
            //right_channel: Vec::new(), 
            right_channel: vec![0.;size],
            head_pos: 0,
            loop_in: 0,
            loop_out: size-1,
            looping: false,
        }
    }

    /// overdub a sample to the current playhead pos, advance playhead
    /// returns the overdub sample
    pub fn overdub(&mut self, left: f32, right: f32) -> (f32, f32) {
        if self.head_pos != self.num_samples() {
            // first overdub left and right values at current head
            self.left_channel[self.head_pos] += left;
            self.right_channel[self.head_pos] += right;

            self.read_unchecked()
        }
        else {
            (0.,0.)
        }
    }

    #[inline]
    fn move_head_unchecked(&mut self) {
        self.head_pos += 1;
        // handle loop bounds
        if self.looping && self.head_pos >= self.loop_out {
            self.head_pos = self.loop_in;
        }
    }

    #[inline]
    fn read_unchecked(&mut self) -> (f32, f32) {
        let samples = (self.left_channel[self.head_pos], self.right_channel[self.head_pos]);
        self.move_head_unchecked();
        samples
    }

    /// read sample at current playhead pos, advance playhead
    /// returns the read sample
    pub fn read(&mut self) -> (f32, f32) {
        if self.head_pos != self.num_samples() {
           self.read_unchecked()
        }
        else { // at end of buffer
            (0., 0.)    
        }
    }

    /// return true if head within loop in and out markers
    #[inline]
    fn within_loop_markers(&self) -> bool {
        self.head_pos >= self.loop_in && self.head_pos <= self.loop_out
    }

    /// reset head to start of buffer
    #[inline]
    pub fn rewind(&mut self) {
        self.head_pos = 0;
    }

    /// reset head to start of loop section or start of buffer
    pub fn gate(&mut self) {
        if self.looping && self.within_loop_markers() {
            self.head_pos = self.loop_in;
        }
        else {
            self.rewind();
        }
    }

    /// set loop in position
    pub fn loop_in(&mut self, offset: usize) {
        if offset <= self.num_samples() {
            self.loop_in = offset;
        }
    }

    /// set loop out position
    pub fn loop_out(&mut self, offset: usize) {
        if offset <= self.num_samples() {
            self.loop_out = offset;
        }
    }

    /// set looping mode
    pub fn looping(&mut self, value: bool) {
        self.looping = value;
    }

    /// return the number of samples in buffer
    pub fn num_samples(&self) -> usize {
        self.left_channel.len()
    }

    /// move forward or backwards along tape
    /// head is only moved if within range
    /// loop markers do not count
    pub fn seek(&mut self, offset: usize, dir: Direction) {
        match dir {
            Direction::Forward if self.head_pos + offset < self.num_samples() => {
                self.head_pos += offset;
            },
            Direction::Backward if self.head_pos>=offset => {
                self.head_pos -= offset;
            },
            _ => { }
        };
    }
}
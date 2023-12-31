use crate::audio_backend::processor_trait::Processor;

pub struct AudioPipeline {
    pub processors: Vec<Box<dyn Processor>>,
}

impl AudioPipeline {
    pub fn new() -> Self {
        AudioPipeline {
            processors: Vec::new(),
        }
    }

    pub fn add_processor(&mut self, processor: Box<dyn Processor>) {
        self.processors.push(processor);
    }

    pub fn remove_processor(&mut self, processor_name: String) {
        self.processors.retain(|x| x.get_name() != processor_name)
    }

    pub fn process_sample(&self, mut sample: f32) -> f32 {
        for processor in &self.processors {
            sample = processor.process(sample)
        }
        sample
    }
}

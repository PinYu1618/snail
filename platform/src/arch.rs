use crate::Boot;
use crate::Cpu;
use crate::Interrupt;
use crate::Memory;
use crate::Timer;

pub trait Arch: Boot + Cpu + Interrupt + Memory + Timer {}
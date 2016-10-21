#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// use std::sync::{Arc, Mutex};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const CONFIG_VALID:       u8 = 0x00;
const CONFIG_TB_PWM_FREQ: u8 = CONFIG_VALID + 4;

const I2C_ADDRESS:        u8 = CONFIG_TB_PWM_FREQ + 4;

const TB_1A_MODE:         u8 = I2C_ADDRESS + 1;
const TB_1A_DIR:          u8 = TB_1A_MODE + 1;
const TB_1A_DUTY:         u8 = TB_1A_DIR + 1;
const TB_1A_SPM_SPEED:    u8 = TB_1A_DUTY + 2;
const TB_1A_SPM_STEP:     u8 = TB_1A_SPM_SPEED + 4;

const TB_1B_MODE:         u8 = TB_1A_SPM_STEP + 4;
const TB_1B_DIR:          u8 = TB_1B_MODE + 1;
const TB_1B_DUTY:         u8 = TB_1B_DIR + 1;
const TB_1B_SPM_SPEED:    u8 = TB_1B_DUTY + 2;
const TB_1B_SPM_STEP:     u8 = TB_1B_SPM_SPEED + 4;

const TB_2A_MODE:         u8 = TB_1B_SPM_STEP + 4;
const TB_2A_DIR:          u8 = TB_2A_MODE + 1;
const TB_2A_DUTY:         u8 = TB_2A_DIR + 1;
const TB_2A_SPM_SPEED:    u8 = TB_2A_DUTY + 2;
const TB_2A_SPM_STEP:     u8 = TB_2A_SPM_SPEED + 4;

const TB_2B_MODE:         u8 = TB_2A_SPM_STEP + 4;
const TB_2B_DIR:          u8 = TB_2B_MODE + 1;
const TB_2B_DUTY:         u8 = TB_2B_DIR + 1;
const TB_2B_SPM_SPEED:    u8 = TB_2B_DUTY + 2;
const TB_2B_SPM_STEP:     u8 = TB_2B_SPM_SPEED + 4;

const SVM1_STATE:         u8 = TB_2B_SPM_STEP + 4;
const SVM1_FREQ:          u8 = SVM1_STATE + 1;
const SVM1_ANGLE:         u8 = SVM1_FREQ + 2;

const SVM2_STATE:         u8 = SVM1_ANGLE + 2;
const SVM2_FREQ:          u8 = SVM2_STATE + 1;
const SVM2_ANGLE:         u8 = SVM2_FREQ + 2;

const SVM3_STATE:         u8 = SVM2_ANGLE + 2;
const SVM3_FREQ:          u8 = SVM3_STATE + 1;
const SVM3_ANGLE:         u8 = SVM3_FREQ + 2;

const SVM4_STATE:         u8 = SVM3_ANGLE + 2;
const SVM4_FREQ:          u8 = SVM4_STATE + 1;
const SVM4_ANGLE:         u8 = SVM4_FREQ + 2;

const SVM5_STATE:         u8 = SVM4_ANGLE + 2;
const SVM5_FREQ:          u8 = SVM5_STATE + 1;
const SVM5_ANGLE:         u8 = SVM5_FREQ + 2;

const SVM6_STATE:         u8 = SVM5_ANGLE + 2;
const SVM6_FREQ:          u8 = SVM6_STATE + 1;
const SVM6_ANGLE:         u8 = SVM6_FREQ + 2;

const IO1_STATE:          u8 = SVM6_ANGLE + 2;
const IO1_MODE:           u8 = IO1_STATE + 1;
const IO1_PUPD:           u8 = IO1_MODE + 1;
const IO1_PPOD:           u8 = IO1_PUPD + 1;

const IO2_STATE:          u8 = IO1_PPOD + 1;
const IO2_MODE:           u8 = IO2_STATE + 1;
const IO2_PUPD:           u8 = IO2_MODE + 1;
const IO2_PPOD:           u8 = IO2_PUPD + 1;

const IO3_STATE:          u8 = IO2_PPOD + 1;
const IO3_MODE:           u8 = IO3_STATE + 1;
const IO3_PUPD:           u8 = IO3_MODE + 1;
const IO3_PPOD:           u8 = IO3_PUPD + 1;

const IO4_STATE:          u8 = IO3_PPOD + 1;
const IO4_MODE:           u8 = IO4_STATE + 1;
const IO4_PUPD:           u8 = IO4_MODE + 1;
const IO4_PPOD:           u8 = IO4_PUPD + 1;

const IO5_STATE:          u8 = IO4_PPOD + 1;
const IO5_MODE:           u8 = IO5_STATE + 1;
const IO5_PUPD:           u8 = IO5_MODE + 1;
const IO5_PPOD:           u8 = IO5_PUPD + 1;

const IO6_STATE:          u8 = IO5_PPOD + 1;
const IO6_MODE:           u8 = IO6_STATE + 1;
const IO6_PUPD:           u8 = IO6_MODE + 1;
const IO6_PPOD:           u8 = IO6_PUPD + 1;



pub enum SelectDCMotor {
    A,
    B,
    C,
    D,
}

pub enum Direction {
    A,
    B,
}

pub struct I2C {
    word: u8,
}

impl I2C {
    fn write(&mut self, word: u8) {
        println!("self.word = {} += word {};", self.word, word);
        self.word += word;
    }
}

#[derive(Clone)]
pub struct MotorCapeInterface {
    interface: Rc<RefCell<I2C>>
}

impl MotorCapeInterface {
    pub fn new() -> MotorCapeInterface {
        MotorCapeInterface {
            interface: Rc::new(RefCell::new(I2C{ word: 0})),
        }
    }

    pub fn write(&self, word: u8) {
        self.interface.borrow_mut().write(word);
    }
}

pub struct DCMotor {
    interface: MotorCapeInterface,
    motor: SelectDCMotor,
    direction: f32,
}

impl DCMotor {
    pub fn new(interface: MotorCapeInterface, motor: SelectDCMotor, direction: Direction) -> DCMotor {
        DCMotor {
            interface: interface,
            motor: motor,
            direction: {
                match direction {
                    Direction::A =>  1.0,
                    Direction::B => -1.0,
                }
            }
        }
    }

    pub fn power(&mut self, pwm: f32) {
        self.interface.write(5);
    }
}

#[cfg(test)]
mod tests {
    use super::{MotorCapeInterface, DCMotor, SelectDCMotor, Direction};
    
    #[test]
    fn single_thread() {
        let interface = MotorCapeInterface::new();
        let mut motor_a = DCMotor::new(interface.clone(), SelectDCMotor::A, Direction::A);
        let mut motor_b = DCMotor::new(interface.clone(), SelectDCMotor::B, Direction::B);
        let mut motor_c = DCMotor::new(interface.clone(), SelectDCMotor::C, Direction::A);
        let mut motor_d = DCMotor::new(interface.clone(), SelectDCMotor::D, Direction::B);
        motor_a.power( 0.1);
        motor_b.power( 0.1);
        motor_c.power(-0.1);
        motor_d.power(-0.1);
        panic!();
    }
}

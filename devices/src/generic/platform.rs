pub mod stm32f3x {

    pub mod adresses {

        pub mod gpio {
            pub const GPIOA_BASE: u32 = 0x4800_0000;
            pub const GPIOB_BASE: u32 = 0x4800_0400;
            pub const GPIOC_BASE: u32 = 0x4800_0800;
            pub const GPIOE_BASE: u32 = 0x4800_1000;
        }
        pub const TIM2_BASEADRESS: u32 = 0x4000_0000;
        pub const TIM3_BASEADRESS: u32 = 0x4000_0400;
        pub const USART1_BASEADRESS: u32 = 0x4001_3800;

        pub mod dma {
            pub const DMA1: u32 = 0x4002_0000;
        }

        // manuel page 55
        pub const RCC: u32 = 0x4002_1000;
    }

    pub mod offsets {
        pub mod dma {
            pub const DMA_ISR: u32 = 0x00;
            pub const DMA_CCR: u32 = 0x08 + 0x0d20;
            pub const DMA_CNDTR: u32 = 0x0C + 0x0d20;
            pub const DMA_CPAR: u32 = 0x10 + 0x0d20;
            pub const DMA_CMAR: u32 = 0x14 + 0x0d20;
        }

        pub mod gpio {
            pub const GPIO_MODER: u32 = 0x00;
            pub const GPIO_OTYPER: u32 = 0x04;
            pub const GPIO_ODR: u32 = 0x14;
            pub const GPIO_AFRL: u32 = 0x20;
            pub const GPIO_AFRH: u32 = 0x24;
        }
        pub mod rcc {
            pub const RCC_APB1RSTR: u32 = 0x10;
            pub const RCC_AHBENR: u32 = 0x14;
            pub const RCC_APB2ENR: u32 = 0x18;
            pub const RCC_APB1ENR: u32 = 0x1C;
        }
        pub mod usart1 {
            pub const BRR: u32 = 0x0C;
            pub const ISR: u32 = 0x1C;
            pub const ICR: u32 = 0x20;
            pub const TDR: u32 = 0x28;
        }
        pub mod tim {
            pub const DIER: u32 = 0x0C;
            pub const SR: u32 = 0x10;
            pub const EGR: u32 = 0x14;
            pub const CNT: u32 = 0x24;
            pub const PSC: u32 = 0x28;
            pub const CCR1: u32 = 0x34;
        }
    }

    pub mod bitfields {
        pub mod gpio {
            pub const INPUT: u32 = 0b00;
            pub const GENERALPURPOSEOUTPUT: u32 = 0b01;
            pub const ALTERNATE: u32 = 0b10;
            pub const ANALOG: u32 = 0b11;
        }
        pub mod rcc {
            pub const SYSCFGEN: u32 = 1;
            pub const IOPAEN: u32 = 17;
            pub const IOPEEN: u32 = 21;
            pub const USART1EN: u32 = 14;
        }
        pub mod usart1 {}
        pub mod tim {
            pub const CEN: u32 = 0b1;
            pub const UG: u32 = 0b1;
            pub const UDIS: u32 = 0b10;
            pub const CC1IE: u32 = 0b10;
        }
    }
}

pub mod stm32f407ve {
    pub mod adresses {
        pub mod gpio {
            // page 65 ref. man
            pub const GPIOA_BASE: u32 = 0x4002_0000;
        }

        // manual page 55
        pub const RCC: u32 = 0x4002_3800;
        pub const USART1_BASEADRESS: u32 = 0x4001_1000;
        // manual page 66
        pub const EXTI: u32 = 0x4001_3C00;
    }

    pub mod bitfields {
        pub mod gpio {
            pub const INPUT: u32 = 0b00;
            pub const GENERALPURPOSEOUTPUT: u32 = 0b01;
            pub const ALTERNATE: u32 = 0b10;
            pub const ANALOG: u32 = 0b11;
        }
        pub mod rcc {
            pub const GPIOPAEN: u32 = 0;
        }
    }
}

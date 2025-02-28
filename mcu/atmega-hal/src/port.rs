pub use avr_hal_generic::port::{mode, PinOps, PinMode};

#[cfg(any(feature = "atmega48p", feature = "atmega168", feature = "atmega328p"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        PORTB: (crate::pac::PORTB, portb, pinb, ddrb),
        PORTC: (crate::pac::PORTC, portc, pinc, ddrc),
        PORTD: (crate::pac::PORTD, portd, pind, ddrd),
    }

    pub struct Pins {
        pb0: PB0 = (crate::pac::PORTB, PORTB, 0, portb, pinb, ddrb),
        pb1: PB1 = (crate::pac::PORTB, PORTB, 1, portb, pinb, ddrb),
        pb2: PB2 = (crate::pac::PORTB, PORTB, 2, portb, pinb, ddrb),
        pb3: PB3 = (crate::pac::PORTB, PORTB, 3, portb, pinb, ddrb),
        pb4: PB4 = (crate::pac::PORTB, PORTB, 4, portb, pinb, ddrb),
        pb5: PB5 = (crate::pac::PORTB, PORTB, 5, portb, pinb, ddrb),
        pb6: PB6 = (crate::pac::PORTB, PORTB, 6, portb, pinb, ddrb),
        pb7: PB7 = (crate::pac::PORTB, PORTB, 7, portb, pinb, ddrb),
        pc0: PC0 = (crate::pac::PORTC, PORTC, 0, portc, pinc, ddrc),
        pc1: PC1 = (crate::pac::PORTC, PORTC, 1, portc, pinc, ddrc),
        pc2: PC2 = (crate::pac::PORTC, PORTC, 2, portc, pinc, ddrc),
        pc3: PC3 = (crate::pac::PORTC, PORTC, 3, portc, pinc, ddrc),
        pc4: PC4 = (crate::pac::PORTC, PORTC, 4, portc, pinc, ddrc),
        pc5: PC5 = (crate::pac::PORTC, PORTC, 5, portc, pinc, ddrc),
        pc6: PC6 = (crate::pac::PORTC, PORTC, 6, portc, pinc, ddrc),
        pd0: PD0 = (crate::pac::PORTD, PORTD, 0, portd, pind, ddrd),
        pd1: PD1 = (crate::pac::PORTD, PORTD, 1, portd, pind, ddrd),
        pd2: PD2 = (crate::pac::PORTD, PORTD, 2, portd, pind, ddrd),
        pd3: PD3 = (crate::pac::PORTD, PORTD, 3, portd, pind, ddrd),
        pd4: PD4 = (crate::pac::PORTD, PORTD, 4, portd, pind, ddrd),
        pd5: PD5 = (crate::pac::PORTD, PORTD, 5, portd, pind, ddrd),
        pd6: PD6 = (crate::pac::PORTD, PORTD, 6, portd, pind, ddrd),
        pd7: PD7 = (crate::pac::PORTD, PORTD, 7, portd, pind, ddrd),
    }
}

#[cfg(feature = "atmega328pb")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        PORTB: (crate::pac::PORTB, portb, pinb, ddrb),
        PORTC: (crate::pac::PORTC, portc, pinc, ddrc),
        PORTD: (crate::pac::PORTD, portd, pind, ddrd),
        PORTE: (crate::pac::PORTE, porte, pine, ddre),
    }

    pub struct Pins {
        pb0: PB0 = (crate::pac::PORTB, PORTB, 0, portb, pinb, ddrb),
        pb1: PB1 = (crate::pac::PORTB, PORTB, 1, portb, pinb, ddrb),
        pb2: PB2 = (crate::pac::PORTB, PORTB, 2, portb, pinb, ddrb),
        pb3: PB3 = (crate::pac::PORTB, PORTB, 3, portb, pinb, ddrb),
        pb4: PB4 = (crate::pac::PORTB, PORTB, 4, portb, pinb, ddrb),
        pb5: PB5 = (crate::pac::PORTB, PORTB, 5, portb, pinb, ddrb),
        pb6: PB6 = (crate::pac::PORTB, PORTB, 6, portb, pinb, ddrb),
        pb7: PB7 = (crate::pac::PORTB, PORTB, 7, portb, pinb, ddrb),
        pc0: PC0 = (crate::pac::PORTC, PORTC, 0, portc, pinc, ddrc),
        pc1: PC1 = (crate::pac::PORTC, PORTC, 1, portc, pinc, ddrc),
        pc2: PC2 = (crate::pac::PORTC, PORTC, 2, portc, pinc, ddrc),
        pc3: PC3 = (crate::pac::PORTC, PORTC, 3, portc, pinc, ddrc),
        pc4: PC4 = (crate::pac::PORTC, PORTC, 4, portc, pinc, ddrc),
        pc5: PC5 = (crate::pac::PORTC, PORTC, 5, portc, pinc, ddrc),
        pc6: PC6 = (crate::pac::PORTC, PORTC, 6, portc, pinc, ddrc),
        pd0: PD0 = (crate::pac::PORTD, PORTD, 0, portd, pind, ddrd),
        pd1: PD1 = (crate::pac::PORTD, PORTD, 1, portd, pind, ddrd),
        pd2: PD2 = (crate::pac::PORTD, PORTD, 2, portd, pind, ddrd),
        pd3: PD3 = (crate::pac::PORTD, PORTD, 3, portd, pind, ddrd),
        pd4: PD4 = (crate::pac::PORTD, PORTD, 4, portd, pind, ddrd),
        pd5: PD5 = (crate::pac::PORTD, PORTD, 5, portd, pind, ddrd),
        pd6: PD6 = (crate::pac::PORTD, PORTD, 6, portd, pind, ddrd),
        pd7: PD7 = (crate::pac::PORTD, PORTD, 7, portd, pind, ddrd),
        pe0: PE0 = (crate::pac::PORTE, PORTE, 0, porte, pine, ddre),
        pe1: PE1 = (crate::pac::PORTE, PORTE, 1, porte, pine, ddre),
        pe2: PE2 = (crate::pac::PORTE, PORTE, 2, porte, pine, ddre),
        pe3: PE3 = (crate::pac::PORTE, PORTE, 3, porte, pine, ddre),
    }
}

#[cfg(feature = "atmega32u4")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        PORTB: (crate::pac::PORTB, portb, pinb, ddrb),
        PORTC: (crate::pac::PORTC, portc, pinc, ddrc),
        PORTD: (crate::pac::PORTD, portd, pind, ddrd),
        PORTE: (crate::pac::PORTE, porte, pine, ddre),
        PORTF: (crate::pac::PORTF, portf, pinf, ddrf),
    }

    pub struct Pins {
        pb0: PB0 = (crate::pac::PORTB, PORTB, 0, portb, pinb, ddrb),
        pb1: PB1 = (crate::pac::PORTB, PORTB, 1, portb, pinb, ddrb),
        pb2: PB2 = (crate::pac::PORTB, PORTB, 2, portb, pinb, ddrb),
        pb3: PB3 = (crate::pac::PORTB, PORTB, 3, portb, pinb, ddrb),
        pb4: PB4 = (crate::pac::PORTB, PORTB, 4, portb, pinb, ddrb),
        pb5: PB5 = (crate::pac::PORTB, PORTB, 5, portb, pinb, ddrb),
        pb6: PB6 = (crate::pac::PORTB, PORTB, 6, portb, pinb, ddrb),
        pb7: PB7 = (crate::pac::PORTB, PORTB, 7, portb, pinb, ddrb),
        pc6: PC6 = (crate::pac::PORTC, PORTC, 6, portc, pinc, ddrc),
        pc7: PC7 = (crate::pac::PORTC, PORTC, 7, portc, pinc, ddrc),
        pd0: PD0 = (crate::pac::PORTD, PORTD, 0, portd, pind, ddrd),
        pd1: PD1 = (crate::pac::PORTD, PORTD, 1, portd, pind, ddrd),
        pd2: PD2 = (crate::pac::PORTD, PORTD, 2, portd, pind, ddrd),
        pd3: PD3 = (crate::pac::PORTD, PORTD, 3, portd, pind, ddrd),
        pd4: PD4 = (crate::pac::PORTD, PORTD, 4, portd, pind, ddrd),
        pd5: PD5 = (crate::pac::PORTD, PORTD, 5, portd, pind, ddrd),
        pd6: PD6 = (crate::pac::PORTD, PORTD, 6, portd, pind, ddrd),
        pd7: PD7 = (crate::pac::PORTD, PORTD, 7, portd, pind, ddrd),
        pe2: PE2 = (crate::pac::PORTE, PORTE, 2, porte, pine, ddre),
        pe6: PE6 = (crate::pac::PORTE, PORTE, 6, porte, pine, ddre),
        pf0: PF0 = (crate::pac::PORTF, PORTF, 0, portf, pinf, ddrf),
        pf1: PF1 = (crate::pac::PORTF, PORTF, 1, portf, pinf, ddrf),
        pf4: PF4 = (crate::pac::PORTF, PORTF, 4, portf, pinf, ddrf),
        pf5: PF5 = (crate::pac::PORTF, PORTF, 5, portf, pinf, ddrf),
        pf6: PF6 = (crate::pac::PORTF, PORTF, 6, portf, pinf, ddrf),
        pf7: PF7 = (crate::pac::PORTF, PORTF, 7, portf, pinf, ddrf),
    }
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        PORTA: (crate::pac::PORTA, porta, pina, ddra),
        PORTB: (crate::pac::PORTB, portb, pinb, ddrb),
        PORTC: (crate::pac::PORTC, portc, pinc, ddrc),
        PORTD: (crate::pac::PORTD, portd, pind, ddrd),
        PORTE: (crate::pac::PORTE, porte, pine, ddre),
        PORTF: (crate::pac::PORTF, portf, pinf, ddrf),
        PORTG: (crate::pac::PORTG, portg, ping, ddrg),
        PORTH: (crate::pac::PORTH, porth, pinh, ddrh),
        PORTJ: (crate::pac::PORTJ, portj, pinj, ddrj),
        PORTK: (crate::pac::PORTK, portk, pink, ddrk),
        PORTL: (crate::pac::PORTL, portl, pinl, ddrl),
    }

    pub struct Pins {
        pa0: PA0 = (crate::pac::PORTA, PORTA, 0, porta, pina, ddra),
        pa1: PA1 = (crate::pac::PORTA, PORTA, 1, porta, pina, ddra),
        pa2: PA2 = (crate::pac::PORTA, PORTA, 2, porta, pina, ddra),
        pa3: PA3 = (crate::pac::PORTA, PORTA, 3, porta, pina, ddra),
        pa4: PA4 = (crate::pac::PORTA, PORTA, 4, porta, pina, ddra),
        pa5: PA5 = (crate::pac::PORTA, PORTA, 5, porta, pina, ddra),
        pa6: PA6 = (crate::pac::PORTA, PORTA, 6, porta, pina, ddra),
        pa7: PA7 = (crate::pac::PORTA, PORTA, 7, porta, pina, ddra),
        pb0: PB0 = (crate::pac::PORTB, PORTB, 0, portb, pinb, ddrb),
        pb1: PB1 = (crate::pac::PORTB, PORTB, 1, portb, pinb, ddrb),
        pb2: PB2 = (crate::pac::PORTB, PORTB, 2, portb, pinb, ddrb),
        pb3: PB3 = (crate::pac::PORTB, PORTB, 3, portb, pinb, ddrb),
        pb4: PB4 = (crate::pac::PORTB, PORTB, 4, portb, pinb, ddrb),
        pb5: PB5 = (crate::pac::PORTB, PORTB, 5, portb, pinb, ddrb),
        pb6: PB6 = (crate::pac::PORTB, PORTB, 6, portb, pinb, ddrb),
        pb7: PB7 = (crate::pac::PORTB, PORTB, 7, portb, pinb, ddrb),
        pc0: PC0 = (crate::pac::PORTC, PORTC, 0, portc, pinc, ddrc),
        pc1: PC1 = (crate::pac::PORTC, PORTC, 1, portc, pinc, ddrc),
        pc2: PC2 = (crate::pac::PORTC, PORTC, 2, portc, pinc, ddrc),
        pc3: PC3 = (crate::pac::PORTC, PORTC, 3, portc, pinc, ddrc),
        pc4: PC4 = (crate::pac::PORTC, PORTC, 4, portc, pinc, ddrc),
        pc5: PC5 = (crate::pac::PORTC, PORTC, 5, portc, pinc, ddrc),
        pc6: PC6 = (crate::pac::PORTC, PORTC, 6, portc, pinc, ddrc),
        pc7: PC7 = (crate::pac::PORTC, PORTC, 7, portc, pinc, ddrc),
        pd0: PD0 = (crate::pac::PORTD, PORTD, 0, portd, pind, ddrd),
        pd1: PD1 = (crate::pac::PORTD, PORTD, 1, portd, pind, ddrd),
        pd2: PD2 = (crate::pac::PORTD, PORTD, 2, portd, pind, ddrd),
        pd3: PD3 = (crate::pac::PORTD, PORTD, 3, portd, pind, ddrd),
        pd4: PD4 = (crate::pac::PORTD, PORTD, 4, portd, pind, ddrd),
        pd5: PD5 = (crate::pac::PORTD, PORTD, 5, portd, pind, ddrd),
        pd6: PD6 = (crate::pac::PORTD, PORTD, 6, portd, pind, ddrd),
        pd7: PD7 = (crate::pac::PORTD, PORTD, 7, portd, pind, ddrd),
        pe0: PE0 = (crate::pac::PORTE, PORTE, 0, porte, pine, ddre),
        pe1: PE1 = (crate::pac::PORTE, PORTE, 1, porte, pine, ddre),
        pe2: PE2 = (crate::pac::PORTE, PORTE, 2, porte, pine, ddre),
        pe3: PE3 = (crate::pac::PORTE, PORTE, 3, porte, pine, ddre),
        pe4: PE4 = (crate::pac::PORTE, PORTE, 4, porte, pine, ddre),
        pe5: PE5 = (crate::pac::PORTE, PORTE, 5, porte, pine, ddre),
        pe6: PE6 = (crate::pac::PORTE, PORTE, 6, porte, pine, ddre),
        pe7: PE7 = (crate::pac::PORTE, PORTE, 7, porte, pine, ddre),
        pf0: PF0 = (crate::pac::PORTF, PORTF, 0, portf, pinf, ddrf),
        pf1: PF1 = (crate::pac::PORTF, PORTF, 1, portf, pinf, ddrf),
        pf2: PF2 = (crate::pac::PORTF, PORTF, 2, portf, pinf, ddrf),
        pf3: PF3 = (crate::pac::PORTF, PORTF, 3, portf, pinf, ddrf),
        pf4: PF4 = (crate::pac::PORTF, PORTF, 4, portf, pinf, ddrf),
        pf5: PF5 = (crate::pac::PORTF, PORTF, 5, portf, pinf, ddrf),
        pf6: PF6 = (crate::pac::PORTF, PORTF, 6, portf, pinf, ddrf),
        pf7: PF7 = (crate::pac::PORTF, PORTF, 7, portf, pinf, ddrf),
        pg0: PG0 = (crate::pac::PORTG, PORTG, 0, portg, ping, ddrg),
        pg1: PG1 = (crate::pac::PORTG, PORTG, 1, portg, ping, ddrg),
        pg2: PG2 = (crate::pac::PORTG, PORTG, 2, portg, ping, ddrg),
        pg3: PG3 = (crate::pac::PORTG, PORTG, 3, portg, ping, ddrg),
        pg4: PG4 = (crate::pac::PORTG, PORTG, 4, portg, ping, ddrg),
        pg5: PG5 = (crate::pac::PORTG, PORTG, 5, portg, ping, ddrg),
        ph0: PH0 = (crate::pac::PORTH, PORTH, 0, porth, pinh, ddrh),
        ph1: PH1 = (crate::pac::PORTH, PORTH, 1, porth, pinh, ddrh),
        ph2: PH2 = (crate::pac::PORTH, PORTH, 2, porth, pinh, ddrh),
        ph3: PH3 = (crate::pac::PORTH, PORTH, 3, porth, pinh, ddrh),
        ph4: PH4 = (crate::pac::PORTH, PORTH, 4, porth, pinh, ddrh),
        ph5: PH5 = (crate::pac::PORTH, PORTH, 5, porth, pinh, ddrh),
        ph6: PH6 = (crate::pac::PORTH, PORTH, 6, porth, pinh, ddrh),
        ph7: PH7 = (crate::pac::PORTH, PORTH, 7, porth, pinh, ddrh),
        pj0: PJ0 = (crate::pac::PORTJ, PORTJ, 0, portj, pinj, ddrj),
        pj1: PJ1 = (crate::pac::PORTJ, PORTJ, 1, portj, pinj, ddrj),
        pj2: PJ2 = (crate::pac::PORTJ, PORTJ, 2, portj, pinj, ddrj),
        pj3: PJ3 = (crate::pac::PORTJ, PORTJ, 3, portj, pinj, ddrj),
        pj4: PJ4 = (crate::pac::PORTJ, PORTJ, 4, portj, pinj, ddrj),
        pj5: PJ5 = (crate::pac::PORTJ, PORTJ, 5, portj, pinj, ddrj),
        pj6: PJ6 = (crate::pac::PORTJ, PORTJ, 6, portj, pinj, ddrj),
        pj7: PJ7 = (crate::pac::PORTJ, PORTJ, 7, portj, pinj, ddrj),
        pk0: PK0 = (crate::pac::PORTK, PORTK, 0, portk, pink, ddrk),
        pk1: PK1 = (crate::pac::PORTK, PORTK, 1, portk, pink, ddrk),
        pk2: PK2 = (crate::pac::PORTK, PORTK, 2, portk, pink, ddrk),
        pk3: PK3 = (crate::pac::PORTK, PORTK, 3, portk, pink, ddrk),
        pk4: PK4 = (crate::pac::PORTK, PORTK, 4, portk, pink, ddrk),
        pk5: PK5 = (crate::pac::PORTK, PORTK, 5, portk, pink, ddrk),
        pk6: PK6 = (crate::pac::PORTK, PORTK, 6, portk, pink, ddrk),
        pk7: PK7 = (crate::pac::PORTK, PORTK, 7, portk, pink, ddrk),
        pl0: PL0 = (crate::pac::PORTL, PORTL, 0, portl, pinl, ddrl),
        pl1: PL1 = (crate::pac::PORTL, PORTL, 1, portl, pinl, ddrl),
        pl2: PL2 = (crate::pac::PORTL, PORTL, 2, portl, pinl, ddrl),
        pl3: PL3 = (crate::pac::PORTL, PORTL, 3, portl, pinl, ddrl),
        pl4: PL4 = (crate::pac::PORTL, PORTL, 4, portl, pinl, ddrl),
        pl5: PL5 = (crate::pac::PORTL, PORTL, 5, portl, pinl, ddrl),
        pl6: PL6 = (crate::pac::PORTL, PORTL, 6, portl, pinl, ddrl),
        pl7: PL7 = (crate::pac::PORTL, PORTL, 7, portl, pinl, ddrl),
    }
}

#[cfg(any(feature = "atmega1284p"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        PORTA: (crate::pac::PORTA, porta, pina, ddra),
        PORTB: (crate::pac::PORTB, portb, pinb, ddrb),
        PORTC: (crate::pac::PORTC, portc, pinc, ddrc),
        PORTD: (crate::pac::PORTD, portd, pind, ddrd),
    }

    pub struct Pins {
        pa0: PA0 = (crate::pac::PORTA, PORTA, 0, porta, pina, ddra),
        pa1: PA1 = (crate::pac::PORTA, PORTA, 1, porta, pina, ddra),
        pa2: PA2 = (crate::pac::PORTA, PORTA, 2, porta, pina, ddra),
        pa3: PA3 = (crate::pac::PORTA, PORTA, 3, porta, pina, ddra),
        pa4: PA4 = (crate::pac::PORTA, PORTA, 4, porta, pina, ddra),
        pa5: PA5 = (crate::pac::PORTA, PORTA, 5, porta, pina, ddra),
        pa6: PA6 = (crate::pac::PORTA, PORTA, 6, porta, pina, ddra),
        pa7: PA7 = (crate::pac::PORTA, PORTA, 7, porta, pina, ddra),
        pb0: PB0 = (crate::pac::PORTB, PORTB, 0, portb, pinb, ddrb),
        pb1: PB1 = (crate::pac::PORTB, PORTB, 1, portb, pinb, ddrb),
        pb2: PB2 = (crate::pac::PORTB, PORTB, 2, portb, pinb, ddrb),
        pb3: PB3 = (crate::pac::PORTB, PORTB, 3, portb, pinb, ddrb),
        pb4: PB4 = (crate::pac::PORTB, PORTB, 4, portb, pinb, ddrb),
        pb5: PB5 = (crate::pac::PORTB, PORTB, 5, portb, pinb, ddrb),
        pb6: PB6 = (crate::pac::PORTB, PORTB, 6, portb, pinb, ddrb),
        pb7: PB7 = (crate::pac::PORTB, PORTB, 7, portb, pinb, ddrb),
        pc0: PC0 = (crate::pac::PORTC, PORTC, 0, portc, pinc, ddrc),
        pc1: PC1 = (crate::pac::PORTC, PORTC, 1, portc, pinc, ddrc),
        pc2: PC2 = (crate::pac::PORTC, PORTC, 2, portc, pinc, ddrc),
        pc3: PC3 = (crate::pac::PORTC, PORTC, 3, portc, pinc, ddrc),
        pc4: PC4 = (crate::pac::PORTC, PORTC, 4, portc, pinc, ddrc),
        pc5: PC5 = (crate::pac::PORTC, PORTC, 5, portc, pinc, ddrc),
        pc6: PC6 = (crate::pac::PORTC, PORTC, 6, portc, pinc, ddrc),
        pc7: PC7 = (crate::pac::PORTC, PORTC, 7, portc, pinc, ddrc),
        pd0: PD0 = (crate::pac::PORTD, PORTD, 0, portd, pind, ddrd),
        pd1: PD1 = (crate::pac::PORTD, PORTD, 1, portd, pind, ddrd),
        pd2: PD2 = (crate::pac::PORTD, PORTD, 2, portd, pind, ddrd),
        pd3: PD3 = (crate::pac::PORTD, PORTD, 3, portd, pind, ddrd),
        pd4: PD4 = (crate::pac::PORTD, PORTD, 4, portd, pind, ddrd),
        pd5: PD5 = (crate::pac::PORTD, PORTD, 5, portd, pind, ddrd),
        pd6: PD6 = (crate::pac::PORTD, PORTD, 6, portd, pind, ddrd),
        pd7: PD7 = (crate::pac::PORTD, PORTD, 7, portd, pind, ddrd),
    }
}

#[cfg(any(feature = "atmega8"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        PORTB: (crate::pac::PORTB, portb, pinb, ddrb),
        PORTC: (crate::pac::PORTC, portc, pinc, ddrc),
        PORTD: (crate::pac::PORTD, portd, pind, ddrd),
    }

    pub struct Pins {
        pb0: PB0 = (crate::pac::PORTB, PORTB, 0, portb, pinb, ddrb),
        pb1: PB1 = (crate::pac::PORTB, PORTB, 1, portb, pinb, ddrb),
        pb2: PB2 = (crate::pac::PORTB, PORTB, 2, portb, pinb, ddrb),
        pb3: PB3 = (crate::pac::PORTB, PORTB, 3, portb, pinb, ddrb),
        pb4: PB4 = (crate::pac::PORTB, PORTB, 4, portb, pinb, ddrb),
        pb5: PB5 = (crate::pac::PORTB, PORTB, 5, portb, pinb, ddrb),
        pb6: PB6 = (crate::pac::PORTB, PORTB, 6, portb, pinb, ddrb),
        pb7: PB7 = (crate::pac::PORTB, PORTB, 7, portb, pinb, ddrb),
        pc0: PC0 = (crate::pac::PORTC, PORTC, 0, portc, pinc, ddrc),
        pc1: PC1 = (crate::pac::PORTC, PORTC, 1, portc, pinc, ddrc),
        pc2: PC2 = (crate::pac::PORTC, PORTC, 2, portc, pinc, ddrc),
        pc3: PC3 = (crate::pac::PORTC, PORTC, 3, portc, pinc, ddrc),
        pc4: PC4 = (crate::pac::PORTC, PORTC, 4, portc, pinc, ddrc),
        pc5: PC5 = (crate::pac::PORTC, PORTC, 5, portc, pinc, ddrc),
        pc6: PC6 = (crate::pac::PORTC, PORTC, 6, portc, pinc, ddrc),
        pd0: PD0 = (crate::pac::PORTD, PORTD, 0, portd, pind, ddrd),
        pd1: PD1 = (crate::pac::PORTD, PORTD, 1, portd, pind, ddrd),
        pd2: PD2 = (crate::pac::PORTD, PORTD, 2, portd, pind, ddrd),
        pd3: PD3 = (crate::pac::PORTD, PORTD, 3, portd, pind, ddrd),
        pd4: PD4 = (crate::pac::PORTD, PORTD, 4, portd, pind, ddrd),
        pd5: PD5 = (crate::pac::PORTD, PORTD, 5, portd, pind, ddrd),
        pd6: PD6 = (crate::pac::PORTD, PORTD, 6, portd, pind, ddrd),
        pd7: PD7 = (crate::pac::PORTD, PORTD, 7, portd, pind, ddrd),
    }
}

#include "tim2_5.h"
#include "rcc.h"
#define BREAK asm("bkpt") 

timerConfiguration_t timer_configurations[5];

void reset_timer(unsigned int tim_nr)
{
    SET_BIT(&((RccRegisterMap_t*) RccBaseAdress)->apb1rstr, 1 << (tim_nr - 2));    
    CLEAR_BIT(&((RccRegisterMap_t*) RccBaseAdress)->apb1rstr, 1 << (tim_nr - 2));    
}

void timer_start(unsigned int tim_nr)
{
    // unsigned int tim_base = get_timx_base(tim_nr);
    SET_BIT(&(timer_configurations[tim_nr].tim_registermap)->cr1, 1 << CEN);
}

void timer_stop(unsigned int tim_nr)
{
    unsigned int tim_base = get_timx_base(tim_nr);
    CLEAR_BIT(&((timer25RegisterMap_t*) tim_base)->cr1, 1 << CEN);
}

void set_udis(unsigned int tim_nr) 
{
    unsigned int tim_base = get_timx_base(tim_nr);   
    WRITE_REGISTER(&((timer25RegisterMap_t*) tim_base)->cr1, READ_REGISTER(&((timer25RegisterMap_t*) tim_base)->cr1) | 1 << UDIS);    
}

void set_ccr(unsigned int tim_nr, unsigned int ccr_value, unsigned int ccr_nr)
{
    unsigned int tim_base = get_timx_base(tim_nr);
    switch (ccr_nr)
    {
    case 1:
        WRITE_REGISTER(&((timer25RegisterMap_t*) tim_base)->ccr1, ccr_value);
        break;
    case 2:
        WRITE_REGISTER(&((timer25RegisterMap_t*) tim_base)->ccr2, ccr_value);
        break;
    case 3:
        WRITE_REGISTER(&((timer25RegisterMap_t*) tim_base)->ccr3, ccr_value);
        break;
    case 4:
        WRITE_REGISTER(&((timer25RegisterMap_t*) tim_base)->ccr4, ccr_value);
        break;
    default:
        break;
    }
}

void generate_ue(unsigned int tim_nr)
{
    unsigned int tim_base = get_timx_base(tim_nr);
    WRITE_REGISTER(&((timer25RegisterMap_t*) tim_base)->egr, READ_REGISTER(&((timer25RegisterMap_t*) tim_base)->egr) | 1);
}


void set_prescaler(unsigned int tim_nr, unsigned int psc_value)
{
    unsigned int tim_base = get_timx_base(tim_nr);
    WRITE_REGISTER(&((timer25RegisterMap_t*) tim_base)->psc, psc_value);
}

void timer_set_sr(unsigned int tim_nr, unsigned int updated_value)
{
    unsigned int tim_base = get_timx_base(tim_nr);
    WRITE_REGISTER(&((timer25RegisterMap_t*) tim_base)->sr, updated_value);
}

unsigned int timer_get_sr(unsigned int tim_nr)
{
    unsigned int tim_base = get_timx_base(tim_nr);
    return READ_REGISTER(&((timer25RegisterMap_t*) tim_base)->sr);
}

unsigned int timer_get_prescaler(__attribute__((unused)) unsigned int tim_nr, unsigned int cycle_length)
{
    unsigned int target_frequency = ((unsigned int) (1.0f / ((float)cycle_length / 1000000.0f)));
    // unsigned int target_frequency = 1;
    // @todo: may over/underflow!!
    return (ahbFrequency / target_frequency) - 1;
}

void timer_enable_pwm()
{
    // @todo: gpio config
    // @todo: enable channels in tim_ccmr - which gpios are associated??
    // @todo: enable capture compare in tim_ccer
    // @todo: gpio config
}


void timer_init(unsigned int tim_nr, __attribute__((unused)) unsigned int arr, unsigned int *ccr, unsigned int cycle_length)
{
    // enable clock
    RccRegisterMap_t* rcc_regs = (RccRegisterMap_t*) RccBaseAdress;
    WRITE_REGISTER(&rcc_regs->apb1enr, READ_REGISTER(&rcc_regs->apb1enr) | 1 << (tim_nr - 2));
    
    reset_timer(tim_nr);
    set_prescaler(tim_nr, timer_get_prescaler(tim_nr, cycle_length));
    generate_ue(tim_nr);
    set_udis(tim_nr);
    for (unsigned int i = 1; i <= 4; i++)
    {
        set_ccr(tim_nr, ccr[i], i);
    }
    timer_flush_counter(tim_nr);

    timer_configurations[tim_nr].tim_nr = tim_nr;
    timer_configurations[tim_nr].tim_registermap = (timer25RegisterMap_t*) get_timx_base(tim_nr);

}

void enable_ccx_ir(unsigned int tim_nr, unsigned int ccr_nr)
{
    unsigned int tim_base = get_timx_base(tim_nr);
    WRITE_REGISTER(&((timer25RegisterMap_t*) tim_base)->dier, READ_REGISTER(&((timer25RegisterMap_t*) tim_base)->dier) | (1 << ccr_nr));
}
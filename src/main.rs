#![no_std]
#![no_main]

type DefaultSerial = arduino_hal::usart::Usart<
    arduino_hal::pac::USART0,
    // TODO: adjust this based on board
    arduino_hal::port::Pin<arduino_hal::port::mode::Input, arduino_hal::hal::port::PE0>,
    arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PE1>
>;

panic_serial::impl_panic_handler!(DefaultSerial);

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let serial = share_serial_port_with_panic(arduino_hal::default_serial!(dp, pins, 57600));

    ufmt::uwrite!(serial, "Hello there from serial!\r\n").unwrap();

    {% case board -%}
      {%- when "Adafruit Trinket" -%}
    let mut led = pins.d1.into_output();
      {%- when
        "Arduino Leonardo",
        "Arduino Mega 2560",
        "Arduino Mega 1280",
        "Arduino Nano",
        "Arduino Nano New Bootloader",
        "Arduino Uno",
        "Nano168",
        "Adafruit Trinket Pro"
      -%}
    let mut led = pins.d13.into_output();
      {%- when "SparkFun ProMicro" -%}
    let mut led = pins.led_rx.into_output();
    {%- endcase %}

    let mut i = 0;

    loop {
        led.toggle();
        arduino_hal::delay_ms(750);

        i += 1;

        if i == 10 {
            panic!("Yo! -- I've blinked a couple of times now - what do you want me to do next?");
        }
    }
}
{%- comment %}
# vim: ft=rust.jinja2
{% endcomment %}

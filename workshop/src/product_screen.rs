use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::styles::*;

    SLEEP_IMG = dep("crate://self/resources/img/sleep.png")
    CART = dep("crate://self/resources/img/cart.png")

    Cart = <RoundedView> {
        padding: 10
        draw_bg: {
            color: #fede67
            radius: 10.
        }
        width: Fit, height: Fit
        image = <RotatedImage> {
            width: 35, height: 35
            source: (CART)
        }
        cart_bubble = <RoundedView>{
            margin: {left: -12}
            width: Fit, height: Fit
            padding: {right: 5, left: 5, top: 2, bottom: 2}
            show_bg: true
            draw_bg: {
                color: #f
                radius: 4
            }
            cart_counter = <Label> {
                text: "0"
                draw_text: {
                    text_style: <TextBold> {
                        font_size: 10.0
                        line_spacing: 1.0
                    }
                    color: #x0
                }
            }
        }
    }

    CounterButton = <ButtonFlat> {
        width: 40, height: 40
        padding: 5
        draw_text: {
            color: #f
            text_style: {
                font_size: 20.0
            }
        }
    }

    ProductScreen = <SectionDown> {
        width: Fill, height: Fill
        align: {x: 0.5, y: 1.0}
        spacing: 40
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                let color_a = #505267;
                let color_b = #x080808;

                let dist = distance(self.pos, vec2(0.5, 0.5));
                let t = smoothstep(0.0, 0.8, dist);
                return mix(color_a, color_b, t);
            }
        }

        <View> {
            padding: {top: 30, right: 40}
            width: Fill, height: Fit
            align: {x: 1.0}
            cart = <Cart> {}
        }

        <Image> {
            width: 350, height: 350
            source: (SLEEP_IMG)
        }
        
        <RoundedView> {
            flow: Down
            width: Fill, height: 250
            show_bg: true
            draw_bg: {
                color: (COLOR_BG)
            }
            padding: 20
            spacing: 5
            name = <Label> {
                text: "Sleep 30\nDissolvable Wafers"
                draw_text: {
                    color: #xf
                    text_style: <TextBold> {
                        font_size: 20.0
                        line_spacing: 1.0
                    }
                }
            }

            qnty = <Label> {
                text: "250mg"
                draw_text: {
                    text_style: {
                        font_size: 10.0
                        line_spacing: 1.0
                    }
                }
            }

            <View> {
                padding: {top: 10}
                flow: Right
                width: Fill, height: Fit
                align: {x: 0.5, y: 0.5}
                spacing: 5
                <Label> {
                    text: "$25.50"
                    draw_text: {
                        color: #xf
                        text_style: <TextBold> {
                            font_size: 20.0
                            line_spacing: 1.0
                        }
                    }
                }
                <View> { width: Fill}
                decrease = <CounterButton> { text: "-" }
                counter = <Label> {
                    text: "0"
                    draw_text: {
                        color: #xf
                        text_style: <TextBold> {
                            font_size: 20.0
                            line_spacing: 1.0
                        }
                    }
                }
                increase = <CounterButton> { text: "+" }
            }
            <Button> {
                width: Fill, height: Fit
                padding: {top: 20, bottom: 20}
                text: "Buy Now"
                draw_text: {
                    text_style: <TextBold> {
                        font_size: 12.0
                        line_spacing: 1.0
                    }
                    color: #f
                }
            }
        }
    }
}

use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::styles::*;

    // TODO: Styling (buttons, background, and image)

    SLEEP_IMG = dep("crate://self/resources/img/sleep.png")
    CART = dep("crate://self/resources/img/cart.png")

    Cart = {{Cart}} <RoundedView> {
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

        animator: {
            shake = {
                default: init,
                init = {
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        image = {
                            draw_bg: { rotation: 0.0}
                        }
                    }
                }

                on = {
                    redraw: true,
                    from: {all: BounceLoop {duration: 0.5, end: 1.0}} 
                    apply: {
                        image = {
                            draw_bg: { rotation: 0.5}
                        }
                    }
                }
            }
        }
    }

    ProductScreen = {{ProductScreen}}<SectionDown> {
        width: Fill, height: Fill
        align: {x: 0.5, y: 1.0}
        spacing: 40
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                let color_a = #494743;
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
                decrease = <Button> {
                    width: Fit, height: Fit
                    text: "-"
                    draw_text: {
                        color: #xf
                        text_style: {
                            font_size: 20.0
                            line_spacing: 1.0
                        }
                    }
                }
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
                increase = <Button> {
                    width: Fit, height: Fit
                    text: "+"
                    draw_text: {
                        color: #xf
                        text_style: {
                            font_size: 20.0
                            line_spacing: 1.0
                        }
                    }
                }
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

#[derive(Live, LiveHook, Widget)]
pub struct ProductScreen {
    #[deref]
    view: View,
    
    #[rust]
    counter: i32
}

impl Widget for ProductScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // TODO: start with label set and then split into counter and cart_counter?
        self.label(id!(counter)).set_text(&self.counter.to_string());
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for ProductScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.button(id!(decrease)).clicked(actions) {
            // TODO: decrement counter, clamp at 0
            self.counter = self.counter.saturating_sub(1);
            self.cart(id!(cart)).update_product_count(cx, self.counter);
            self.redraw(cx);
        }

        if self.button(id!(increase)).clicked(actions) {
            self.counter += 1;
            self.cart(id!(cart)).update_product_count(cx, self.counter);
            self.redraw(cx);
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Cart {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for Cart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.view.redraw(cx);
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // if self.animator.need_init() || self.animator_in_state(cx, id!(shake.init)) {
        //     self.animator_play(cx, id!(shake.on));
        // }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl CartRef {
    fn update_product_count(&mut self, cx: &mut Cx, new_count: i32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.label(id!(cart_counter)).set_text(&new_count.to_string());

            if new_count > 0 {
                if !inner.animator_in_state(cx, id!(shake.on)) {
                    inner.animator_play(cx, id!(shake.on));
                }

                inner.view(id!(cart_bubble)).apply_over(cx, live!{
                    draw_bg: {
                        color: #e74c3c
                    }
                });
    
                inner.label(id!(cart_counter)).apply_over(cx, live!{
                    draw_text: {
                        color: #f
                    }
                });
            } else {
                inner.animator_play(cx, id!(shake.init));

                inner.view(id!(cart_bubble)).apply_over(cx, live!{
                    draw_bg: {
                        color: #f
                    }
                });
    
                inner.label(id!(cart_counter)).apply_over(cx, live!{
                    draw_text: {
                        color: #x0
                    }
                });
            }
        }
    }
}

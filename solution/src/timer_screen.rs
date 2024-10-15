use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::styles::*;

    BEACH = dep("crate://self/resources/img/covers/beach.jpg")
    ICON_PLAY = dep("crate://self/resources/img/icons/play.svg")
    ICON_STOP = dep("crate://self/resources/img/icons/stop.svg")

    ControlButton = <Button> {
        width: 65, height: 65
        draw_icon: {
            icon_walk: {width: 12, height: 12}
            instance color: #fff
            fn get_color(self) -> vec4 {
                return (COLOR_ICON_SECONDARY);
            }
        }

        icon_walk: {width: 15, height: 15}
    }

    Preset = <RoundedView> {
        width: Fill, height: Fit
        cursor: Hand
        flow: Right
        show_bg: true
        draw_bg: {
            color: (COLOR_BG)
        }
        align: {x: 0.5, y: 0.5}
        padding: {left: 20, right: 20, top: 15, bottom: 15}
        content = <SectionDown> {
            spacing: 5.0
            align: {x: 0.0, y: 0.5}

            title = <Label> {
                text: "Plank"
                draw_text: {
                    text_style: <TextBold> {
                        font_size: 12.0
                    }
                    color: #f
                }
            }

            subtitle = <Label> {
                text: "1 minute"
                draw_text: {
                    text_style: {
                        font_size: 9
                    }
                    color: (COLOR_TEXT_SECONDARY)
                }
            }
        }

        play_button = <ControlButton> {
            draw_icon: {
                svg_file: (ICON_PLAY)
            }
        }

        pause_button = <ControlButton> {
            visible: false
            draw_icon: {
                svg_file: (ICON_STOP)
            }
        }
    }

    TimerScreen = {{TimerScreen}} {
        flow: Down
        width: Fill, height: Fill
        align: {x: 0.5, y: 0.0}
        padding: {top: 100, left: 20, right: 20, bottom: 25}

        show_bg: true
        draw_bg: {
            color: (COLOR_BG)
        }

        // Timer
        // TODO: add animation
        <RoundedView> {
            padding: 10
            width: 250, height: 250
            align: {x: 0.5, y: 0.5}
            draw_bg: {
                radius: 60.0
                // color: #x0
                // color: #c9a0ff
                color: #ff9a62
                border_width: 4.0
                border_color: (COLOR_BG)
            }

            timer = <Label> {
                text: "00:00"
                draw_text: {
                    text_style: <TextBold> {
                        font_size: 25.0
                    }
                    color: #f
                }
            }
        }

        controls = <SectionRight> {
            visible: false
            width: Fill, height: Fit
            align: {x: 0.5, y: 0.5}
            margin: {bottom: 30}
            spacing: 120

            cancel_button = <Button> {
                width: Fit, height: Fit
                padding: {left: 20, right: 20, top: 10, bottom: 10}
                text: "Cancel"
                draw_text: {
                    text_style: {
                        font_size: 14
                    }
                    color: #e
                }
            }

            pause_button = <Button> {
                width: Fit, height: Fit
                padding: {left: 20, right: 20, top: 10, bottom: 10}
                text: "Pause"
                draw_text: {
                    text_style: {
                        font_size: 14
                    }
                    color: #e
                }
            }
        }

        <SectionDown> {
            spacing: 10
            <View> {
                padding: {left: 20}
                width: Fill, height: Fit
                <Label> {
                    text: "Abs workout"
                    draw_text: {
                        text_style: <TextBold> {
                            font_size: 18
                        }
                        color: #e
                    }
                }
            }
            planks = <Preset> {}
            crunches = <Preset> { 
                content = { 
                    title = { text: "Crunches" }
                    subtitle = { text: "45 seconds" }
                }
            }
            leg_raises = <Preset> { 
                content = { 
                    title = { text: "Leg raises" }
                    subtitle = { text: "30 seconds" }
                }
            }
        }
        
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TimerScreen {
    #[deref]
    view: View,

    #[rust]
    timer: Option<Timer>,
    #[rust]
    total_time: Option<f32>,

    #[animator]
    animator: Animator,
}

impl Widget for TimerScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator.handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        self.view.handle_event(cx, event, scope);
        self.match_event(cx, event);
        if let Some(timer) = self.timer {
            if timer.is_event(event).is_some() {
                // if self.animator_in_state(cx, id!(blink.off)) {
                //     self.animator_play(cx, id!(blink.on));
                // } else {
                //     self.animator_play(cx, id!(blink.off));
                // }
                
                self.total_time = Some(self.total_time.unwrap() - 0.08);
                if self.total_time <= Some(0.05) {
                    cx.stop_timer(timer);
                }
                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(seconds) = self.total_time { 
            // Format time as MM:SS:MS
            let total_millis = (seconds * 1000.0).round() as u64;
            let minutes = total_millis / 60000;
            let seconds = (total_millis % 60000) / 1000;
            let milliseconds = (total_millis % 1000) / 10;

            let formatted_time = format!("{:02}:{:02}:{:02}", minutes, seconds, milliseconds);
            self.label(id!(timer)).set_text(&formatted_time);
        }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for TimerScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut total_duration = 0.0;
        let mut should_start_timer = false;

        // Cancel button
        if self.button(id!(cancel_button)).clicked(actions) {
            self.view(id!(controls)).set_visible(false);
            self.total_time = None;
            self.timer = None;
            self.redraw(cx);
            self.label(id!(timer)).set_text("00:00");
        }

        // Pause button
        if self.button(id!(pause_button)).clicked(actions) {
            // Stop the current timer
            if let Some(timer) = self.timer {
                cx.stop_timer(timer);
                self.timer = None;
                self.button(id!(pause_button)).set_text("Resume");
            } else {
                // Resume timer
                self.timer = Some(cx.start_interval(0.08));
                self.button(id!(pause_button)).set_text("Pause");
            }
            self.redraw(cx);
        }

        // Presets
        if self.button(id!(planks.play_button)).clicked(actions) {
            total_duration = 60.0;
            should_start_timer = true;
            self.view(id!(planks)).apply_over(cx, live!{ draw_bg: { color: #x333645 } });
            log!("planks play button clicked");
            self.button(id!(planks.play_button)).set_visible(false);
            self.button(id!(planks.pause_button)).set_visible(true);
        }

        if self.button(id!(planks.stop_button)).clicked(actions) {
            cx.stop_timer(self.timer.unwrap());
            self.timer = None;
            should_start_timer = false;
            log!("STOP button clicked");
            // self.view(id!(planks)).apply_over(cx, live!{ draw_bg: { color: #x333645 } });
            self.button(id!(planks.play_button)).set_visible(true);
            self.button(id!(planks.pause_button)).set_visible(false);
        }

        if self.button(id!(crunches.play_button)).clicked(actions) {
            total_duration = 45.0;  
            should_start_timer = true;
            self.view(id!(crunches)).apply_over(cx, live!{ draw_bg: { color: #x333645 } });
            self.button(id!(crunches.play_button)).set_visible(false);
            self.button(id!(crunches.pause_button)).set_visible(true);
        }

        if self.button(id!(leg_raises.play_button)).clicked(actions) {
            total_duration = 30.0;
            should_start_timer = true;
            self.view(id!(leg_raises)).apply_over(cx, live!{ draw_bg: { color: #x333645 } });
            self.button(id!(leg_raises.play_button)).set_visible(false);
            self.button(id!(leg_raises.pause_button)).set_visible(true);
        }

        if should_start_timer {
            self.view(id!(controls)).set_visible(true);
            self.total_time = Some(total_duration);
            self.timer = Some(cx.start_interval(0.08));
            self.redraw(cx);
        }
    }
}
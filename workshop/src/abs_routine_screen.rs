use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::styles::*;

    ICON_PLAY = dep("crate://self/resources/img/icons/play.svg")
    ICON_STOP = dep("crate://self/resources/img/icons/stop.svg")

    ControlButton = <Button> {
        width: 45, height: 45
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
        flow: Right
        show_bg: true
        draw_bg: {
            color: (COLOR_BG)
        }
        align: {x: 0.5, y: 0.5}
        padding: {left: 20, right: 20, top: 10, bottom: 10}
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

        stop_button = <ControlButton> {
            visible: false
            draw_icon: {
                svg_file: (ICON_STOP)
            }
        }
    }

    AbsRoutineScreen = {{AbsRoutineScreen}} {
        flow: Down
        width: Fill, height: Fill
        align: {x: 0.5, y: 0.0}
        padding: {top: 100, left: 20, right: 20, bottom: 25}

        show_bg: true
        draw_bg: {
            color: (COLOR_BG)
        }

        // Timer
        <RoundedView> {
            padding: 10
            width: 250, height: 250
            align: {x: 0.5, y: 0.5}
            draw_bg: {
                border_radius: 60.0
                color: #ff9a62
                border_size: 4.0
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

        // Workout items
        <SectionDown> {
            spacing: 10
            margin: {top: 20}
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
pub struct AbsRoutineScreen {
    #[deref]
    view: View,
}

impl Widget for AbsRoutineScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for AbsRoutineScreen {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::styles::*;

    FIRE = dep("crate://self/resources/img/fire.png")

    Routine = <RoundedView> {
        flow: Down
        align: {x: 0.5, y: 0.5}
        padding: 10
        width: 150, height: Fit
        show_bg: true
        draw_bg: {color: #c55367}
        // draw_bg: {color: #x0}

        <RoundedView> {
            padding: 10
            margin: {bottom: 10}
            width: Fit, height: Fit
            show_bg: true
            draw_bg: { color: #a53b57 }
            <Image> {
                source: (FIRE)
                width: 40, height: 40
            }
        }

        <Label> {
            text: "Chest"
            draw_text: {
                color: #f
                text_style: <TextBold> {
                    line_spacing: 1
                    font_size: 20
                }
            }
        }

        <Label> {
            text: "Calories"
            draw_text: {
                color: #f
                text_style: {
                    line_spacing: 1
                    font_size: 12
                }
            }
        }
    }

    Exercise = <RoundedView> {
        cursor: Hand
        show_bg: true
        draw_bg: {
            color: #333645
            // dark green
            // color: #b6f36a
        }
        align: {x: 0.5, y: 0.5}
        width: Fill, height: Fit
        padding: 10
        width: Fill, height: Fit
        content = <SectionDown> {
            width: Fill, height: Fit
            header = <SectionRight> {
                width: Fill, height: Fit
                <RoundedView> {
                    height: 20, width: 8
                    show_bg: true
                    draw_bg: {
                        color: #b6f36a
                        radius: 2
                    }
                }
                title = <Label> {
                    text: "Push-ups"
                    draw_text: {
                        color: #xf
                        text_style: <TextBold> {
                            font_size: 12
                        }
                    }
                }
                <View> {width: 10}
                <Label> {
                    text: "15 x 3"
                    draw_text: {
                        color: #xf
                        text_style: <TextBold> {
                            font_size: 12
                        }
                    }
                }

                // <CheckBox> { text: "Push Ups"}
            }
            <View> {
                width: Fill, height: Fit
                <Label> {
                    text: "Biceps, triceps and shoulders"
                    draw_text: {
                        color: #c
                        text_style: {
                            font_size: 10
                        }
                    }
                }
            }
            <SectionRight> {
                width: Fill, height: Fit
                <SectionDown> {
                    spacing: 0
                    width: Fit, height: Fit
                    set_1_chbox = <CheckBox> { text: "Set 1"}
                    set_2_chbox = <CheckBox> { text: "Set 2"}
                    set_3_chbox = <CheckBox> { text: "Set 3"}
                }
                <View> { width: 50 }
                done_button = <Button> {
                    height: Fill
                    width: Fit
                    text: "Complete All"
                    draw_text: {
                        color: #f
                        text_style: <TextBold> {
                            font_size: 12
                        }
                    }
                }
            }
        }
        // show_bg: true
        // draw_bg: {
        //     #333645
        // }
    }

    ExerciseScreen = {{ExerciseScreen}} {
        flow: Down, spacing: 10
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: (COLOR_BG)
        }

        padding: {top: 100} // TODO: this shouldnt be necessary
        padding: {left: 25, right: 25}


        <Label> {
            text: "Today's Activity"
            draw_text: {
                color: #f
                text_style: <TextBold> {
                    font_size: 15
                }
            }
        }

        <Label> {
            text: "You have a full body workout today!"
            draw_text: {
                color: #f
                text_style: {
                    font_size: 12
                }
            }
        }

        // <ScrollXView> {
        //     height: 150
        //     spacing: 15
        //     <Routine> {}
        //     <Routine> {}
        //     <Routine> {}
        //     <Routine> {}
        // }

        <RoundedView> {
            margin: {top: 20}
            spacing: 10
            flow: Right
            width: Fill, height: 500

            calories = <RoundedView> {
                flow: Down
                align: {x: 0.5, y: 0.5}
                width: 100, height: Fill
                show_bg: true
                draw_bg: {color: #c55367}

                <RoundedView> {
                    padding: 10
                    margin: {bottom: 30}
                    width: Fit, height: Fit
                    show_bg: true
                    draw_bg: { color: #a53b57 }
                    <Image> {
                        source: (FIRE)
                        width: 40, height: 40
                    }
                }

                calories_number = <Label> {
                    text: "0"
                    draw_text: {
                        color: #f
                        text_style: <TextBold> {
                            line_spacing: 1
                            font_size: 20
                        }
                    }
                }

                <Label> {
                    text: "Calories"
                    draw_text: {
                        color: #f
                        text_style: {
                            line_spacing: 1
                            font_size: 12
                        }
                    }
                }
            }
            items = <SectionDown> {
                width: Fill, height: Fit
                pushups = <Exercise> {}
                squats = <Exercise> {
                    content = { header = { title = { text: "Squats"} }}
                }
                lunges = <Exercise> {
                    content = { header = { title = { text: "Lunges"} }}
                }
            }
        }
    }
}

#[derive(Widget, Live, LiveHook)]
pub struct ExerciseScreen {
    #[deref]
    view: View,

    #[rust]
    calories: usize,
}

impl Widget for ExerciseScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for ExerciseScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let pushups_button = self.button(id!(pushups.done_button));
        let pushups_set_1_chbox = self.check_box(id!(pushups.set_1_chbox));    
        let pushups_set_2_chbox = self.check_box(id!(pushups.set_2_chbox));
        let pushups_set_3_chbox = self.check_box(id!(pushups.set_3_chbox));

        let squats_button = self.button(id!(squats.done_button));
        let squats_set_1_chbox = self.check_box(id!(squats.set_1_chbox));
        let squats_set_2_chbox = self.check_box(id!(squats.set_2_chbox));
        let squats_set_3_chbox = self.check_box(id!(squats.set_3_chbox));
        
        let lunges_button = self.button(id!(lunges.done_button));
        let lunges_set_1_chbox = self.check_box(id!(lunges.set_1_chbox));
        let lunges_set_2_chbox = self.check_box(id!(lunges.set_2_chbox));
        let lunges_set_3_chbox = self.check_box(id!(lunges.set_3_chbox));


        // Pushups
        if pushups_button.clicked(actions) {
            self.calories += 300;
            self.label(id!(calories_number))
                .set_text_and_redraw(cx, &format!("{0}", self.calories));

            self.view(id!(pushups)).apply_over(cx, live! {
                draw_bg: {color: #x2a2c36 }
            });

            // pushups_button.set_text_and_redraw(cx, "Done");
            // pushups_button.set_enabled(false);
            pushups_set_1_chbox.set_selected(cx, true);
            pushups_set_2_chbox.set_selected(cx, true);
            pushups_set_3_chbox.set_selected(cx, true); 
        }


        // Squats
        if squats_button.clicked(actions) {
            self.calories += 200;
            self.label(id!(calories_number))
                .set_text_and_redraw(cx, &format!("{0}", self.calories));

            self.view(id!(squats)).apply_over(cx, live! {
                draw_bg: {color: #x2a2c36 }
            });

            squats_set_1_chbox.set_selected(cx, true);
            squats_set_2_chbox.set_selected(cx, true);
            squats_set_3_chbox.set_selected(cx, true);
        }

        // Lunges
        if lunges_button.clicked(actions) {
            self.calories += 150;
            self.label(id!(calories_number))
                .set_text_and_redraw(cx, &format!("{0}", self.calories));

            self.view(id!(lunges)).apply_over(cx, live! {
                draw_bg: {color: #x2a2c36 }
            });

            lunges_set_1_chbox.set_selected(cx, true);
            lunges_set_2_chbox.set_selected(cx, true);
            lunges_set_3_chbox.set_selected(cx, true);
        }
    }
}

use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    
    import crate::styles::*;
    import crate::home::*;
    import crate::daily_workout_screen::*;
    import crate::abs_routine_screen::*;
    import crate::product_screen::*;

    App = {{App}} {
        ui: <Window> {
            window: {position: vec2(970, 80), inner_size: vec2(440, 800)},

            body = {
                show_bg: true
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        // gradient to make it slightly darker at the very bottom
                        // Define the amount of darkening at the bottom (e.g., 5% darker)
                        let darkened_color = (COLOR_BG_DARKER);

                        // Use smoothstep for a smoother gradient transition
                        let t = smoothstep(0.8, 1.0, self.pos.y);

                        // Mix the colors based on the interpolated value 't'
                        return mix(COLOR_BG, darkened_color, t);
                    }
                }
                nav = <StackNavigation> {
                    root_view = <Home> {}
                    product_view = <StackNavigationView> {
                        header = {
                            // padding: {left: 20.}
                            content = {
                                title_container = {
                                    title = {
                                        text: "Store"
                                    }
                                }
                            }
                        }
                        body = <ProductScreen> {}
                    }
                    daily_workout_view = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: "Exercise"
                                    }
                                }
                            }
                        }
                        body = <DailyWorkoutScreen> {}
                    }
                    abs_routine_view = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: "Timer"
                                    }
                                }
                            }
                        }
                        body = <AbsRoutineScreen> {}
                    }
                }
            }
        }
    }
}

// Registers App as the main module for this application
app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

/// Registers all the live components for this application
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::styles::live_design(cx);
        crate::home::live_design(cx);
        crate::abs_routine_screen::live_design(cx);
        crate::daily_workout_screen::live_design(cx);
        crate::product_screen::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut navigation = self.ui.stack_navigation(id!(nav));
        navigation.handle_stack_view_actions(cx, &actions);

        if let Some(_) = self.ui.view(id!(store_section)).finger_down(&actions) {
            cx.widget_action(
                self.ui.widget_uid(),
                &Scope::default().path,
                StackNavigationAction::NavigateTo(live_id!(product_view)),
            );
        }

        if let Some(_) = self.ui.view(id!(daily_workout)).finger_down(&actions) {
            cx.widget_action(
                self.ui.widget_uid(),
                &Scope::default().path,
                StackNavigationAction::NavigateTo(live_id!(daily_workout_view)),
            );
        }

        if let Some(_) = self.ui.view(id!(abs_routine)).finger_down(&actions) {
            cx.widget_action(
                self.ui.widget_uid(),
                &Scope::default().path,
                StackNavigationAction::NavigateTo(live_id!(abs_routine_view)),
            );
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        self.match_event(cx, event);
    }
}

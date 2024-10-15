use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::styles::*;

    JUMPING_IMG = dep("crate://self/resources/img/jumping.png")
    STRETCHING_IMG = dep("crate://self/resources/img/stretching.png")

    ProgramScreen = <SectionDown> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                // a gradient from center outwards
                let color_b = #7fb7f3;
                let color_a = #c3f0c9;

                let dist = distance(self.pos, vec2(0.2, 0.5));

                let t = smoothstep(0.0, 0.8, dist);

                return mix(color_a, color_b, t);


                // // Use smoothstep for a smoother gradient transition
                // let t = smoothstep(0.0, 1.0, self.pos.y);

                // // Mix the colors based on the interpolated value 't'
                // return mix(color_a, color_b, t);
            }
        }
        <SectionRight> {
            <View> {
                margin: {left: -80.}
                width: Fit, height: Fill
                align: {x: 0.5, y: 0.5}
                <Image> {
                    width: 300, height: Fit
                    fit: Biggest
                    source: (JUMPING_IMG)   
                }
            }
            <SectionDown> {
                align: {x: 0.0, y: 0.5}
                spacing: 0.0
                <Label> {
                    text: "The\nPower\nStart"
                    draw_text: {
                        text_style: <TextBold> {
                            line_spacing: 0.7
                            font_size: 30.0
                        }
                        color: #x2
                    }
                }

                <Label> {
                    text: "Hira Riaz"
                    draw_text: {
                        text_style: <TextBold> {
                            font_size: 12.0
                        }
                        color: #x4
                    }
                }

                <RoundedView> {
                    margin: {top: 30.}
                    width: 100, height: 40
                    align: {x: 0.5, y: 0.5}
                    <Label> {
                        text: "Start"
                        draw_text: {
                            text_style: <TextBold> {
                                font_size: 12.0
                            }
                            color: #x4
                        }
                    }
                }
                <View> {
                    width: Fit, height: Fit
                    align: {x: 0.0, y: 0.5}
                    flow: Down
                    <RoundedView> {
                        padding: 15
                        flow: Down
                        spacing: 10
                        margin: {top: 30.}
                        align: {x: 0.5, y: 0.5}
                        width: Fit, height: Fit
                        show_bg: true
                        draw_bg: {
                            radius: 10.
                            color: #bee2e6
                        }
                        <Image> {
                            width: 80, height: Fit
                            fit: Biggest
                            source: (STRETCHING_IMG)   
                        }
                    }
                    
                    <Label> {
                        padding: {top: 5, right: 5.}
                        text: "Next"
                        draw_text: {
                            text_style: {
                                font_size: 10.0
                            }
                            color: #x4
                        }
                    }
                    <Label> {
                        padding: {right: 5.}
                        text: "Streaching"
                        draw_text: {
                            text_style: <TextBold> {
                                font_size: 10.0
                            }
                            color: #x4
                        }
                    }
                }
            }
        }
    }
}
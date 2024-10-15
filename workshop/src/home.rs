use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::styles::*;

    GOLD = dep("crate://self/resources/img/covers/gold.jpg")
    SLEEP_IMG = dep("crate://self/resources/img/sleep.png")

    BOTTLE = dep("crate://self/resources/img/bottle.png")
    CARROT = dep("crate://self/resources/img/carrot.png")
    DUMBELL = dep("crate://self/resources/img/dumbell.png")
    FIRE = dep("crate://self/resources/img/fire.png")
    FIRE_DARK = dep("crate://self/resources/img/fire_dark.png")
    SNEAKER = dep("crate://self/resources/img/sneaker.png")

    HomeHeader = <View> {
        flow: Down
        width: Fill,
        height: Fit
        <Label> { 
            padding: {left: 8}
            text: "Good morning,", 
            draw_text: {
                // color: #9,
                text_style: <TextBold> {
                    font_size: 15.0
                }
            } 
        }
        <Label> { 
            padding: {left: 8}
            text: "Benjamin", 
            draw_text: {
                color: #f,
                text_style: <TextBold> {
                    font_size: 15.0
                }
            } 
        }
    }

    Box = <RoundedView> {
        cursor: Hand
        draw_bg: {
            radius: 5
        }
        padding: 20
        flow: Down
        width: Fill
        align: {x: 0.0, y: 0.0}

        header = <SectionRight> {
            icon = <Image> {
                width: 20, height: 20
            }
            title = <Label> {
                draw_text: {
                    // color: #222430,
                    color: #3a,
                    text_style: { //<TextBold> {
                        font_size: 12.0
                    }
                }
            }
        }
    }

    Pill = <RoundedView> {
        width: 80, height: 30
        align: {x: 0.5, y: 0.5}
        draw_bg: {
            radius: 7.0
            border_width: 1.0
            border_color: #3
        }
        lbl = <Label> {
            draw_text: {
                text_style: <TextBold> {}
                // color: #x7
            }
        }
    }

    SleepBox = <Box> {
        draw_bg: {
            color: #c9a0ff
        }
        padding: 20
        align: {x: 0.0, y: 0.0}
        height: 120 
        flow: Down
        header = {
            icon = { source: (CARROT) }
            title = { text: "Sleep" }
        }

        <View> { height: Fill }
        <Label> { 
            text: "Time in Bed"
            draw_text: {
                color: #4a,
                text_style: {
                    font_size: 10.0
                }
            }
        }
        <SectionRight> {
            <Label> { 
                text: "5hr 15min"
                draw_text: {
                    color: #2a,
                    text_style: {
                        font_size: 15.0
                    }
                }
            }
        }
    }

    NutritionBox = <Box> {
        draw_bg: {
            color: #fede67
        }
        height: 150
        header = {
            icon = { source: (CARROT) }
            title = { text: "Nutrition" }
        }
    }

    StepsBox = <Box> {
        draw_bg: {
            color: #ff9a62
        }
        height: 100
        header = {
            icon = { source: (SNEAKER) }
            title = { text: "Steps" }
        }

        <SectionRight> {
            show_bg: true
            spacing: 0
            align: {x: 0.0, y: 1.0}
            <Label> {
                text: "7.500 / "
                draw_text: {
                    color: #2a,
                    text_style: {
                        font_size: 15.0
                    }
                }
            }
            <Label> {
                text: "15k goal"
                draw_text: {
                    color: #2a,
                    text_style: {
                        font_size: 10.0
                    }
                }
            }
        }
    }

    HabitsBox = <Box> {
        draw_bg: {
            color: #94dbfb
        }
        height: 100
        header = {
            icon = { source: (BOTTLE) }
            title = { text: "Habit tracker" }
        }
    }

    WorkoutBox = <Box> {
        draw_bg: {
            color: #b6f36a
        }
        height: 150
        header = {
            icon = { source: (DUMBELL) }
            title = { text: "Workout" }
        }
    }

    Store = <SectionDown> {
        margin: {top: 30}
        spacing: 25.
        width: Fill, height: Fit
        <SectionDown> {
            width: Fill, height: Fit
            <SectionRight> {
                align: {x: 0.5, y: 0.5}
                width: Fill, height: Fit
                spacing: 20.0
                <SectionDown> {
                    width: Fill, height: Fit
                    spacing: 2
                    <Label> {
                        text: "LATEST IN OUR STORE", 
                        draw_text: {
                            color: #d
                            text_style: {
                                font_size: 9.0
                            }
                        } 
                    }
                    <Label> { 
                        draw_text: {
                            color: #f,
                            text_style: <TextBold> {
                                font_size: 14.0
                            }
                        } 
                    }
                }
            }
    
            store_section = <RoundedView> {
                cursor: Hand
                draw_bg: { 
                    color: #333645 
                    radius: 5
                }
                height: 100
                align: {x: 0.0, y: 0.5}
                padding: {left: 20, right: 20}
                flow: Right, spacing: 15
                img = <Image> {
                    source: (SLEEP_IMG)
                    width: 70, height: 70
                }
                description = <SectionDown> {
                    height: Fit
                    align: {x: 0.0, y: 0.5}
                    spacing: 4
                    <Label> {
                        text: "Improve your sleep"
                        draw_text: {
                            text_style: <TextBold> {
                                font_size: 12.0
                            }
                        }
                    }
        
                    subtitle = <Label> {
                        text: "Melatonin tablets now available"
                        draw_text: {
                            text_style: {
                                font_size: 9.0
                            }
                        }
                    }
                }
            }
        }
    }

    Home = <View> {
        padding: {top: 35, left: 20, right: 20, bottom: 25}
        flow: Down
        spacing: 5.0

        HomeHeader = <HomeHeader> {}

        pills = <SectionRight> {
            height: Fit, width: Fill
            align: {x: 0.5, y: 0.5}
            padding: 10
            spacing: 20
            <Pill> { lbl = { text: "Daily", draw_text: { color: #f } }, draw_bg: { color: #759cff } }
            <Pill> { lbl = { text: "Weekly" } }
            <Pill> { lbl = { text: "Monthly" } }
            <Pill> { lbl = { text: "Yearly" } }
        }

        feed = <SectionDown> {
            padding: 10
            margin: {top: 10}
            height: Fit
            show_bg: true

            <SleepBox> {}
        }
    }
}

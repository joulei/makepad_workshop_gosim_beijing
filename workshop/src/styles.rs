use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    
    TextBold = {
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Bold.ttf")}
    }

    COLOR_BG = #232531
    COLOR_BG_DARKER = #211d20
    COLOR_BLUE = #759cff
        
    COLOR_TEXT_PRIMARY = #f
    COLOR_TEXT_SECONDARY = #b

    COLOR_ICON = #6f7d95
    COLOR_ICON_SECONDARY = #a1b0c9

    SectionRight = <View> { flow: Right }
    SectionDown = <View> { flow: Down }

    RoundedImage = <Image> {
        draw_bg: {
            instance radius: 10.0
            instance border_width: 0.0
            instance border_color: #f

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let size = self.rect_size;
                let center = size * 0.5;
                let radius = min(min(self.radius, size.x * 0.5), size.y * 0.5);
    
                sdf.box(0., 0., size.x, size.y, radius);
                
                // Apply the image color
                let image_color = sample2d(self.image, self.pos);
                sdf.fill_keep(image_color);
    
                // Apply border if specified
                if self.border_width > 0.0 {
                    sdf.stroke(self.border_color, self.border_width);
                }
    
                return sdf.result;
            }
        }
    }

    StackNavigationView = <StackNavigationView> {
        header = {
            content = {
                button_container = {
                    padding: {left: 20}
                }
            }
        }
    }
}

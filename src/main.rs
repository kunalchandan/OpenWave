slint::include_modules!();

slint::slint! {
    export component LPFWindow {
        width : 128px;
        height : 128px;
        Rectangle {
            x: 0px;
            y: 0px;
            Path {
                width : parent.width;
                height: parent.height;
                stroke: blueviolet;
                stroke-width: 4px;
                MoveTo {
                    x:0;
                    y:parent.height/2/1px;
                }
                LineTo {
                    x:40;
                    y:32;
                }
                LineTo {
                    x:44;
                    y:25;
                }
                Close {
                }
            }
        }
    }
}

fn main() {
    let _ = dotenvy::dotenv();
    // LPFWindow::new().unwrap().run().unwrap();
    Keyboard::new().unwrap().run().unwrap();
}

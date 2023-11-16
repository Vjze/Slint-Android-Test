#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: i_slint_backend_android_activity::AndroidApp) {
    use std::cell::RefCell;
    use std::rc::Rc;

    use tokio::runtime::Runtime;
    slint::platform::set_platform(Box::new(
        i_slint_backend_android_activity::AndroidPlatform::new(app),
    ))
    .unwrap();

    // ... rest of your code ...
    // slint::slint!{
    //     export component MainWindow inherits Window {
    //         Text { text: "Hello World"; }
    //     }
    // }
    let app = MainWindow::new().unwrap();
    let weak = app.as_weak();
    let operator = Rc::new(RefCell::new(String::new()));
    app.global::<Lgo>().on_login_clicked(move |logininfos|{
        let _ = weak
            .upgrade_in_event_loop(move |ui| {
                let rt = Runtime::new().unwrap();
                rt.block_on(async move{
                    let r = login_user(logininfos.name.to_string(), logininfos.password.to_string()).await;
                    ui.set_is_login(r);
                    return;
                });
                return;
            });
            return;
    });
    let weak_window = app.as_weak();
    app.global::<Lgo>().on_login_cancel(move || {
        let window = weak_window.unwrap();
        window.hide().unwrap();
    });
    
    app.run().unwrap();
}

use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub async fn login_user(id: String, pa: String) -> bool {
    let mut client = login_check().await;
    let i = id.clone();
    let stream = client
        .simple_query(format!("select name from login where name = '{}'", i))
        .await
        .unwrap();
    let row = stream.into_results().await.unwrap();
    let r = row.get(0).unwrap();

    if r.is_empty() {
        false
    } else {
        login_pass(id, pa).await
    }
}

pub async fn login_pass(id: String, p: String) -> bool {
    let mut client = login_check().await;
    let i = id.clone();
    let stream = client
        .simple_query(format!("select password from login where name = '{}'", i))
        .await
        .unwrap();
    let row = stream.into_row().await.unwrap().unwrap();
    let r = row.get::<&str, _>(0).unwrap().to_string();
    r == p
}
pub async fn login_check() -> Client<Compat<TcpStream>> {
    let mut config = Config::new();
    config.host("192.168.2.189");
    config.port(1433);
    config.database("login");
    config.authentication(AuthMethod::sql_server("hztest", "hztest"));
    config.trust_cert();
    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    tcp.set_nodelay(true).unwrap();

    tiberius::Client::connect(config, tcp.compat_write())
        .await
        .unwrap()
}
// UI
slint::slint! {
    import { GridBox , Button, LineEdit} from "std-widgets.slint";

    export struct LoginInfos {
    name:string,
    password:string,
}

    export global Lgo {
    callback login_clicked(LoginInfos);
    callback login_cancel;
}

   
    export component Login inherits Rectangle {
        VerticalLayout {
          padding: 20px;
          alignment: center;
          spacing: 10px;
            HorizontalLayout{
              padding: 50px;
              alignment:center;
              spacing: 30px;
                Text {text : @tr("账号：");
                font-size: 14px;
                font-weight: 650;}
                name_input := LineEdit { 
                  width: 40%;
                    placeholder-text : @tr("输入账号....");
                    font-size : 14px;
                 }
            }
            HorizontalLayout{
              padding: 20px;
              alignment: center;
              spacing: 30px;
                Text {text : @tr("密码：");
                font-size: 14px;
                font-weight: 650;}
                password_input := LineEdit { 
                  width: 40%;
                    placeholder-text : @tr("输入密码....");
                    font-size: 14px;
                    input-type: password;
                }
            }
            HorizontalLayout {
              padding: 50px;
              alignment: center;
              spacing: 100px;
              Button { 
                  width: 70px;
                  height: 32px;
                  text: "取消";
                  clicked => {
                    Lgo.login-cancel()
                  }
               }
               Button {
                width: 70px;
                  height: 32px;
                  text: "登录";
                  clicked => {
                    Lgo.login_clicked({
                      name:name_input.text,
                      password:password-input.text,
                    })
                  }
               }
            }
           
        }
    }
    export component QueryPage inherits Rectangle {
        Text {
            text: "hello word";
        }
        
    }
    export component MainWindow inherits Window {
        in-out property <bool> is-login :false ;
        if root.is-login == false : Login {}
        if root.is-login == true :  QueryPage { }
        
    }
}
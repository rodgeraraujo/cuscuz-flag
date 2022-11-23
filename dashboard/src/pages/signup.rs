use yew::prelude::*;

use crate::components::email_input::EmailInput;
use crate::components::password_input::PasswordInput;

pub enum SignUpMsg {
    SetEmail(String),
    SetPassword(String),
    Submit,
}

#[derive(Debug, Default)]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
}

impl Component for SignUpForm {
    type Message = SignUpMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::SetEmail(new_email_value) => self.email = new_email_value,
            Self::Message::SetPassword(new_password_value) => self.password = new_password_value,
            Self::Message::Submit => {
                // Let's send email + password to backend
                unimplemented!()
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change_email = ctx.link().callback(SignUpMsg::SetEmail);
        let on_change_password = ctx.link().callback(SignUpMsg::SetPassword);
        let onclick = ctx.link().callback(|_| SignUpMsg::Submit);

        html! {
            <div class="columns">
                <div class="column is-half is-offset-one-quarter">
                    <div class="field is-grouped is-grouped-centered">
                        <p class="title is-2">{ "Sign Up" }</p>
                    </div>
                    <div class="field">
                        <label class="label">{ "Email" }</label>
                        <div class="control">
                            <EmailInput on_change={on_change_email} value={self.email.clone()} />
                        </div>
                        <label class="label">{ "Password" }</label>
                        <div class="control">
                            <PasswordInput on_change={on_change_password} value={self.password.clone()} />
                        </div>
                    </div>
                    <div class="field is-grouped is-grouped-centered">
                        <button {onclick} class="button is-warning is-light" type="submit">{ "Get started" }</button>
                    </div>
                </div>
            </div>
        }
    }
}
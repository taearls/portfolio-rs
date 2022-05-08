use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use weblog::console_log;
use yew::{html, Component, Context, FocusEvent, Html, TargetCast};

use std::fmt;
use std::fmt::{Display, Formatter};

use crate::components::{ErrorMessage, HeadingOne, Page, Paragraph, SuccessMessage};

pub struct Contact {
    form_data: ContactFormData,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ContactFormData {
    name: String,
    email: String,
    subject: String,
    message: String,
    errors: Vec<ContactFormError>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum RequiredField {
    Name,
    Email,
    Message,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ContactFormError {
    RequiredField(RequiredField),
    InvalidEmail,
    TargetCastError,
}

impl Display for ContactFormError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            ContactFormError::InvalidEmail => "invalid email",
            ContactFormError::TargetCastError => "target cast error",
            ContactFormError::RequiredField(required_field_variant) => match required_field_variant
            {
                RequiredField::Name => "name is required",
                RequiredField::Email => "email is required",
                RequiredField::Message => "message is required",
            },
        };
        write!(f, "{message}")
    }
}

pub enum ContactMsg {
    FormSubmitted,
    NameChanged(String),
    SubjectChanged(String),
    EmailChanged(String),
    MessageChanged(String),
    Error(ContactFormError),
}

impl Component for Contact {
    type Properties = ();
    type Message = ContactMsg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            form_data: ContactFormData::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let ContactFormData {
            name,
            email,
            subject,
            message,
            errors,
            ..
        } = &self.form_data;

        match msg {
            ContactMsg::FormSubmitted => {
                console_log!("name is: ", name);
                console_log!("email is: ", email);
                console_log!("subject is: ", subject);
                console_log!("message is: ", message);
                errors.iter().for_each(|error| {
                    console_log!("error: ", error.to_string());
                });

                // send email

                // maybe I should clear out form data after it's sent
                // self.form_data = ContactFormData::default();
                false
            }
            ContactMsg::NameChanged(name) => {
                self.form_data
                    .errors
                    .retain(|x| x != &ContactFormError::RequiredField(RequiredField::Name));
                self.form_data.name = name;
                true
            }
            ContactMsg::SubjectChanged(subject) => {
                self.form_data.subject = subject;
                true
            }
            ContactMsg::EmailChanged(email) => {
                self.form_data.errors.retain(|x| {
                    x != &ContactFormError::InvalidEmail
                        && x != &ContactFormError::RequiredField(RequiredField::Email)
                });
                self.form_data.email = email;
                true
            }
            ContactMsg::MessageChanged(message) => {
                self.form_data
                    .errors
                    .retain(|x| x != &ContactFormError::RequiredField(RequiredField::Message));
                self.form_data.message = message;
                true
            }
            ContactMsg::Error(error_variant) => {
                let mut cleaning_up_email_errors = false;
                if error_variant == ContactFormError::InvalidEmail {
                    if let Some(error_pos) =
                        self.form_data.errors.iter().position(|x| {
                            *x == ContactFormError::RequiredField(RequiredField::Email)
                        })
                    {
                        self.form_data.errors.swap_remove(error_pos);
                        cleaning_up_email_errors = true;
                    }
                }
                if error_variant == ContactFormError::RequiredField(RequiredField::Email) {
                    if let Some(error_pos) = self
                        .form_data
                        .errors
                        .iter()
                        .position(|x| *x == ContactFormError::InvalidEmail)
                    {
                        self.form_data.errors.swap_remove(error_pos);
                        cleaning_up_email_errors = true;
                    }
                }
                if !self.form_data.errors.contains(&error_variant) {
                    self.form_data.errors.push(error_variant);
                    true
                } else {
                    cleaning_up_email_errors
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onsubmit = link.callback(|e: FocusEvent| {
            e.prevent_default();
            ContactMsg::FormSubmitted
        });
        let on_name_change = link.callback(|e: FocusEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                match validate_required_field(&input.value()) {
                    Some(name) => ContactMsg::NameChanged(name.to_string()),
                    None => ContactMsg::Error(ContactFormError::RequiredField(RequiredField::Name)),
                }
            } else {
                ContactMsg::Error(ContactFormError::TargetCastError)
            }
        });

        let on_email_change = link.callback(|e: FocusEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                match validate_required_field(&value) {
                    Some(email) => match validate_email(email) {
                        Some(email) => ContactMsg::EmailChanged(email.to_string()),
                        None => ContactMsg::Error(ContactFormError::InvalidEmail),
                    },
                    None => {
                        ContactMsg::Error(ContactFormError::RequiredField(RequiredField::Email))
                    }
                }
            } else {
                ContactMsg::Error(ContactFormError::TargetCastError)
            }
        });

        let on_message_change = link.callback(|e: FocusEvent| {
            let input = e.target_dyn_into::<HtmlTextAreaElement>();

            if let Some(input) = input {
                match validate_required_field(&input.value()) {
                    Some(message) => ContactMsg::MessageChanged(message.to_string()),
                    None => {
                        ContactMsg::Error(ContactFormError::RequiredField(RequiredField::Message))
                    }
                }
            } else {
                ContactMsg::Error(ContactFormError::TargetCastError)
            }
        });

        let on_subject_change = link.callback(|e: FocusEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                let subject = input.value();
                ContactMsg::SubjectChanged(subject)
            } else {
                ContactMsg::Error(ContactFormError::TargetCastError)
            }
        });
        html! {
            <Page>
                <HeadingOne>
                    {"Contact Tyler Earls"}
                </HeadingOne>
                <Paragraph>
                    {"If you're interested in hiring me for coding work, my music, or just want to say hello—I'd love to hear from you. I'm a voracious learner, and nothing is too nerdy or niche for my taste."}
                </Paragraph>
                <Paragraph>
                    {"The best way to reach me is by completing the form below."}
                </Paragraph>
                <form
                    id="contact"
                    class="form-boxshadow mx-auto my-8 bg-gray-200 dark:bg-gray-900 rounded-md w-full max-w-sm"
                    method="POST"
                    enctype="text/plain"
                    {onsubmit}
                >
                    <fieldset
                        class="px-4 py-2"
                    >
                        <div>
                            <label
                                class="block text-purple-700 dark:text-purple-400 font-bold mb-1 md:mb-0 pr-4"
                                for="contactName"
                            >
                                {"Name"}
                                <span>{" *"}</span>
                            </label>
                            <input
                                id="contactName"
                                onblur={on_name_change}
                                class="form-input w-full mb-2 text-soft-black placeholder-gray-600 focus:bg-white focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                                type="text"
                                name="name"
                            />
                            <ErrorMessage show={self.form_data.errors.contains(&ContactFormError::RequiredField(RequiredField::Name))}>
                                {"Please enter your name."}
                            </ErrorMessage>
                        </div>

                        <div
                            class="flex flex-col"
                            // :class="{'mb-2': !$v.email.text.$error}"
                        >
                            <label
                                class="block text-purple-700 dark:text-purple-400 font-bold mb-1 md:mb-0 pr-4"
                                for="contactEmail"
                            >
                                {"Email"}
                                <span>{" *"}</span>
                            </label>
                            <input
                                id="contactEmail"
                                // v-model.trim="email.text"
                                class="form-input w-full text-soft-black placeholder-gray-600 focus:bg-white focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                                type="email"
                                name="email"
                                required={true}
                                onblur={on_email_change}
                                placeholder="beammeup@scotty.com"
                                // @input="$v.email.text.$reset(); isUserTyping = true;"
                                // @blur="$v.email.text.$touch(); isUserTyping = false;"
                            />
                            <ErrorMessage show={self.form_data.errors.contains(&ContactFormError::RequiredField(RequiredField::Email))}>
                                {"Please enter your email."}
                            </ErrorMessage>
                            <ErrorMessage show={self.form_data.errors.contains(&ContactFormError::InvalidEmail)}>
                                {"Please enter a valid email address."}
                            </ErrorMessage>
                        </div>

                        <div>
                            <label
                                class="block text-purple-700 dark:text-purple-400 font-bold mb-1 md:mb-0 pr-4"
                                for="contactSubject"
                            >
                                {"Subject"}</label>
                            <input
                                id="contactSubject"
                                // v-model.trim="subject.text"
                                class="form-input w-full mb-2 text-soft-black placeholder-gray-600 focus:bg-white focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                                type="text"
                                name="subject"
                                onblur={on_subject_change}
                                // :placeholder="subject.placeholder"
                            />
                        </div>
                        <div
                            class="flex flex-col"
                            // :class="{'mb-4': !$v.message.text.$error}"
                        >
                            <label
                                class="block text-purple-700 dark:text-purple-400 font-bold mb-1 md:mb-0 pr-4"
                                for="contactMessage"
                            >
                                {"Message "}<span>{" *"}</span>
                            </label>
                            <textarea
                                id="contactMessage"
                                class="form-textarea w-full h-32 text-soft-black placeholder-gray-600 focus:bg-white focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                                name="message"
                                required={true}
                                onblur={on_message_change}
                            />
                            <ErrorMessage show={self.form_data.errors.contains(&ContactFormError::RequiredField(RequiredField::Message))}>
                                {"Please enter a message."}
                            </ErrorMessage>
                        </div>
                        // <vue-recaptcha
                        //     :key="shouldCompactRecaptcha + prefersDarkMode"
                        //     :sitekey="$config.recaptchaSitekey"
                        //     :theme="prefersDarkMode ? 'dark' : 'light'"
                        //     :size="shouldCompactRecaptcha ? 'compact' : 'normal'"
                        //     :load-recaptcha-script="true"
                        //     @verify="recaptchaVerified = true"
                        //     @expired="recaptchaVerified = false"
                        // />

                        <div class="relative">
                            <SuccessMessage>
                                {"Thank you. I look forward to working with you!"}
                            </SuccessMessage>
                            // <client-only>
                            //     <loading-state v-if="requestState === 'loading'" />
                            // </client-only>
                            // <SuccessMessage>
                            //     {"Success! I'll be in touch shortly."}
                            // </SuccessMessage>
                            <ErrorMessage>
                                {"There was an error sending your message. Please try again."}
                            </ErrorMessage>

                            <div class="flex items-center">
                                <input
                                    type="submit"
                                    value="Send Email"
                                    disabled={is_form_disabled(&self.form_data)}
                                    // :disabled="saveDisabled"
                                    class="inline-block my-2 cursor-pointer text-white transition-colors transition-padding ease-in-out duration-200 bg-purple-700 dark:bg-purple-400 rounded-lg pl-2 pr-10 disabled:pr-2 disabled:cursor-not-allowed disabled:opacity-50 focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                                    // :class="{'submit-hover': !saveDisabled && hoveringMessage }"
                                    // @mouseover="hoveringMessage = !saveDisabled"
                                    // @mouseleave="hoveringMessage = false"
                                />
                                // <transition name="draw">
                                // <right-arrow-icon
                                //     v-if="!saveDisabled"
                                //     style="margin-left: -32px;"
                                // />
                                // </transition>
                            </div>
                        </div>
                    </fieldset>
                </form>
            </Page>
        }
    }
}

fn is_form_disabled(form_data: &ContactFormData) -> bool {
    // in order of precedence:
    // 1. UI error messages present
    // 2. email address invalid
    // 3. required fields empty
    !form_data.errors.is_empty()
        || validate_email(&form_data.email).is_none()
        || required_values_empty(form_data)
}

fn required_values_empty(form_data: &ContactFormData) -> bool {
    let ContactFormData {
        name,
        email,
        message,
        ..
    } = form_data;
    name.is_empty() || email.is_empty() || message.is_empty()
}

fn validate_required_field(value: &str) -> Option<&str> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn validate_email(value: &str) -> Option<&str> {
    lazy_static! {
        static ref EMAIL_REGEX: Regex =
            Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
                .unwrap();
    }
    if !value.is_empty() && EMAIL_REGEX.is_match(value) {
        Some(value)
    } else {
        None
    }
}

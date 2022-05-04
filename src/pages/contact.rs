use yew::{html, Component, Context, FocusEvent, Html};

pub struct Contact {
    form_data: ContactFormData,
}

#[derive(Default)]
pub struct ContactFormData {
    name: String,
    email: String,
    subject: Option<String>,
    message: String,
}

pub enum ContactMsg {
    FormSubmitted,
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
        match msg {
            ContactMsg::FormSubmitted => {
                println!("name is: {}", self.form_data.name);
                println!("email is: {}", self.form_data.email);
                if let Some(subject) = &self.form_data.subject {
                    println!("subject is: {}", subject);
                } else {
                    println!("subject is empty!");
                }
                println!("message is: {}", self.form_data.message);

                self.form_data = ContactFormData::default();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onsubmit = link.callback(|e: FocusEvent| {
            e.prevent_default();
            ContactMsg::FormSubmitted
        });
        html! {
            <section class="px-4 mx-auto max-w-none w-4/5 leading-8">
                <h1 class="text-center mb-4 text-purple-700 dark:text-purple-400 font-extrabold text-4xl leading-tight">
                    {"Contact Tyler Earls"}
                </h1>
                <p class="w-full max-w-lg mx-auto text-justify text-soft-black dark:text-white my-4 text-lg md:text-xl leading-normal">
                    {"If you're interested in hiring me for coding work, my music, or just want to say hello—I'd love to hear from you. I'm a voracious learner, and nothing is too nerdy or niche for my taste."}
                </p>
                <p class="w-full max-w-lg mx-auto text-justify text-soft-black dark:text-white my-4 text-lg md:text-xl leading-normal">
                    {"The best way to reach me is by completing the form below."}
                </p>
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
                                // v-model.trim="name.text"
                                // :style="$v.name.text.$error ? 'margin: 0' : ''"
                                class="form-input w-full mb-2 text-soft-black placeholder-gray-600 focus:bg-white focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                                type="text"
                                name="name"
                                // :placeholder="name.placeholder"
                                // @input="$v.name.text.$reset(); isUserTyping = true;"
                                // @blur="$v.name.text.$touch(); isUserTyping = false;"
                            />
                            <p
                                // v-if="$v.name.text.$error"
                                class="error-message"
                            >
                                {"Please enter your name."}
                            </p>
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
                            placeholder="beammeup@scotty.com"
                            // @input="$v.email.text.$reset(); isUserTyping = true;"
                            // @blur="$v.email.text.$touch(); isUserTyping = false;"
                        />
                        <p
                            // v-if="$v.email.text.$dirty && !$v.email.text.required"
                            class="error-message"
                        >
                            {"Please enter your email."}
                        </p>
                        <p
                            // v-if="$v.email.text.$dirty && !$v.email.text.emailValidationRegex"
                            class="error-message"
                        >
                            {"Please enter a valid email address."}
                        </p>
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
                                // v-model.trim="message.text"
                                class="form-textarea w-full h-32 text-soft-black placeholder-gray-600 focus:bg-white focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                                name="message"
                                required={true}
                                // :placeholder="message.placeholder"
                                // @input="$v.message.text.$reset(); isUserTyping = true;"
                                // @blur="$v.message.text.$touch(); isUserTyping = false;"
                            />
                            <p
                                // v-if="$v.message.text.$error"
                                class="error-message"
                            >
                                {"Please enter a message."}
                            </p>
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
                            <p
                                // v-if="!saveDisabled"
                                class="success-message"
                            >
                                {"Thank you. I look forward to working with you!"}
                            </p>
                            // <client-only>
                            //     <loading-state v-if="requestState === 'loading'" />
                            // </client-only>
                            <p
                                // v-if="requestState === 'success'"
                                class="success-message"
                            >
                                {"Success! I'll be in touch shortly."}
                            </p>
                            <p
                                // v-if="requestState === 'error'"
                                class="error-message"
                            >
                                {"There was an error sending your message. Please try again."}
                            </p>

                            <div class="flex items-center">
                                <input
                                    type="submit"
                                    value="Send Email"
                                    // :disabled="saveDisabled"
                                    class="inline-block my-2 text-white transition-colors transition-padding ease-in-out duration-200 bg-purple-700 dark:bg-purple-400 rounded-lg pl-2 pr-10 disabled:pr-2 disabled:cursor-not-allowed disabled:opacity-50 focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
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
            </section>
        }
    }
}

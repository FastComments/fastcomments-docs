[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

By default, FastComments will render the comment widget in the locale determined by the user's system and browser.

When a user comments or logs in, we update their last used locale and use this for sending emails, as well.

This impacts how the commenting widget is translated for the user. Locale consists of the user's language and region, so configuring locale will
usually change the language used to show text to the user.

#### Via The UI

This can be defined using the widget customization UI. See the "Locale / Language" option:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

This can be overridden with a desired locale.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Supported Languages and Locale Codes

0[You can find the complete list of supported languages and the corresponding locale codes here.](/guide-supported-languages.html#supported-languages)

### SSO Note

If you're using SSO, you might want to pass the user's locale in the user object, so that emails and other things are localized correctly for them.

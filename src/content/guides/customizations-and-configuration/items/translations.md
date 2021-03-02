[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

With FastComments, all text in the comment widget is customizable.

You can override a single piece of text, like the submit button, or all text in the entire comment widget.

By default, the text in the comment widget is translated based on the user's locale. However, we can override the text, if we are confident
that our user-base is using the same local/language, for example:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

All customizable translations can be found <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">here</a> under the "advanced options" tab.

However, there is an easier way, via the widget customization UI. In there, we can simply find the text that shows in the commenting widget in the EN_US locale, and specify
a replacement.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

All translations overrides currently affect all locales.

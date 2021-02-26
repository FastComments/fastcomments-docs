[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

By default, FastComments will render the comment widget in the locale determined by the user's system and browser.

This impacts how the commenting widget is translated for the user.

This can be overridden with a desired locale. Please check <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L53" target="_blank">here</a> for valid locales.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

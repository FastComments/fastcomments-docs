[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

By default, FastComments will render the comment widget in the locale determined by the user's system and browser.

When a user comments or logs in, we update their last used locale and use this for sending emails, as well.

This impacts how the commenting widget is translated for the user.

#### Via The UI

This can be defined using the widget customization UI. See the "Locale / Language" option:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

This can be overridden with a desired locale. Please check <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L142" target="_blank">here</a> for valid locales.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Locale Codes

All locale codes are:

- `de_de` German (Germany)
- `en_us` English (North America)
- `es_es` Spanish (Spain)
- `fr_fr` French (France)
- `it_it` Italian (Italy)
- `ja_jp` Japanese (Japan)
- `ko_kr` Korean (Korea)
- `pl_pl` Polish (Poland)
- `pt_br` Portuguese (Brazilian)
- `ru_ru` Russian (Russia)
- `tr_tr` Turkish (Turkey)
- `zh_cn` Chinese (Simplified)
- `zh_tw` Chinese (Traditional)

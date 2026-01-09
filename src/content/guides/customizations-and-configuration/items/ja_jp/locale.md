[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

既定では、FastCommentsはユーザーのシステムとブラウザによって決定されたロケールでコメントウィジェットを表示します。

ユーザーがコメントしたりログインしたりすると、そのユーザーが最後に使用したロケールを更新し、メール送信にもそのロケールを使用します。

これはコメントウィジェットがユーザー向けにどのように翻訳されるかに影響します。ロケールはユーザーの言語と地域で構成されるため、ロケールを設定すると通常、ユーザーに表示されるテキストの言語が変更されます。

#### Via The UI

これはウィジェットのカスタマイズUIで定義できます。「Locale / Language」オプションを参照してください：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

これを任意のロケールで上書きできます。

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Supported Languages and Locale Codes

[サポートされている言語と対応するロケールコードの完全な一覧はここで確認できます。](/guide-supported-languages.html#supported-languages)

### SSO に関する注意

SSOを使用している場合、ユーザーオブジェクトにユーザーのロケールを渡しておくと、メールなどが正しくローカライズされます。
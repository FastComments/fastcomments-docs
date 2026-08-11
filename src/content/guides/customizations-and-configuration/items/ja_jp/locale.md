[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

デフォルトでは、FastComments はユーザーのシステムとブラウザで決定されたロケールでコメントウィジェットを表示します。

ユーザーがコメントしたりログインしたりすると、最後に使用したロケールを更新し、メール送信にもこのロケールを使用します。

これにより、コメントウィジェットの翻訳がユーザー向けにどのように行われるかが影響を受けます。ロケールはユーザーの言語と地域で構成されるため、ロケールを設定すると通常、ユーザーに表示されるテキストの言語が変更されます。

#### UI から

ウィジェットカスタマイズ UI を使用して設定できます。「ロケール / 言語」オプションをご覧ください：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='ウィジェットカスタマイズページのロケール/言語ドロップダウンは、訪問者の検出されたロケールを上書きするために使用されます'; title='ロケール/言語の変更' app-screenshot-end]

#### コードから

希望するロケールで上書きできます。

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### サポートされている言語とロケールコード

[サポートされている言語と対応するロケールコードの完全なリストは、こちらで確認できます。](/guide-supported-languages.html#supported-languages)

### SSO に関する注意

SSO を使用している場合、ユーザーオブジェクトにユーザーのロケールを渡すと、メールやその他の項目が正しくローカライズされます。

---
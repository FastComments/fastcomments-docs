---
[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

認証のために、FastCommentsはブラウザでサードパーティのクッキーが有効になっていることに依存します。これが無効だと、ユーザーは常にコメントする際に
メールアドレスを入力する必要があり（メール入力欄が非表示の場合を除く）、コメントは常に未検証として表示されます（デフォルト設定）。

これを回避するには、サードパーティ・クッキーバイパスを有効にできます。 

この設定を有効にすると、ユーザーがログインされていることを示すメッセージを表示する小さなポップアップが表示されます。 このポップアップ
はユーザーがコメントウィジェットとやり取りするたびに表示されます。例えば、コメントを投稿したときなどです。

コードでは、**enableThirdPartyCookieBypass** フラグを true に設定することでこれを行えます:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

また、ウィジェットカスタマイズのUIで `Enable Third-Party Cookie Popup` の項目から設定することもできます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---
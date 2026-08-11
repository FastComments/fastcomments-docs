[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

認証のために、FastComments はブラウザでサードパーティークッキーが有効になっていることに依存しています。これが無効だと、ユーザーは常にコメントする際にメールアドレスを入力しなければならず（メール入力フィールドが非表示でない限り）、コメントはデフォルトで未確認として表示されます。

この問題を回避するには、サードパーティークッキー バイパスを有効にできます。

この設定を有効にすると、ユーザーがログイン中であることを示すメッセージを表示する小さなポップアップが表示されます。このポップアップは、ユーザーがコメントウィジェットとやり取りするたびに表示されます。たとえば、コメントを残すときなどです。

コードでこれを行うには、**enableThirdPartyCookieBypass** フラグを true に設定します：

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'サードパーティークッキー バイパスの有効化'; code-example-end]

ウィジェットカスタマイズ UI でも、`Enable Third-Party Cookie Popup` の下でこの設定を行うことができます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='「Enable Third-Party Cookie Popup」チェックボックスがチェックされたウィジェットカスタマイズページ'; title='サードパーティークッキー バイパスの有効化' app-screenshot-end]
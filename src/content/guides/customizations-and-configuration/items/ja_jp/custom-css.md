[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments はカスタマイズ可能になるよう設計されています。コメントウィジェット自体はセキュリティ上の理由で iframe 内で実行されるため、カスタムスタイリングを適用するには以下の 2 つのアプローチのいずれかに従う必要があります。

最初の、最も簡単なアプローチであり、当社が推奨する方法は、[ウィジェットカスタマイズページ](https://fastcomments.com/auth/my-account/customize-widget) を使用することです。

ウィジェットカスタマイズページで「高度なオプションを表示」セクションを確認すると、そこに「Custom CSS」とラベル付けされた領域があります：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='ウィジェットカスタマイズページの「高度なオプションを表示」下のカスタム CSS エディタ'; title='カスタム CSS 入力エリア' app-screenshot-end]

このアプローチにはいくつかの利点があります：
1. 入力された CSS はユーザーに送信される前に圧縮され、編集 UI でのフォーマットが一貫して保たれます。
2. ウィジェットカスタマイズ UI のすべての利点を活用でき、例えばサイトごとにコメントウィジェットを簡単にカスタマイズできます。
3. コメントウィジェットに変更を加える際、カスタムスタイリングはリリースプロセスの一部としてテストされます。

2 番目のアプローチは、ウィジェット設定で **customCSS** パラメータを指定する方法です。例：

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

ただし、これには *制限* があります：
1. ヘッダーサイズの制限により、サーバーがリクエストを拒否するまでに渡せるカスタム CSS の量に上限があります。
2. カスタム CSS をインフラストラクチャやビルドシステムで管理する必要があります。これはデメリットというよりメリットになることもあります。
3. この使用ケースでは、カスタム CSS をネットワーク上で **2 回** 送信する追加のオーバーヘッドがあります（サーバーに送信され、iframe コンテンツとして返されます）。ただし、ほとんどのペイロードサイズでは目立ちません。
4. 一般的な最適化として CSS を圧縮してネットワーク上のサイズを削減しますが、このアプローチでは自分で処理する必要があります。
5. コメントウィジェットに変更が加わった際、カスタム CSS はテストされません。

### 外部 CSS ファイル

`@import` を使用してウィジェットに外部ファイルを取得させることができます！

`@import` はカスタマイズルール内に入れることが推奨されます。こうすれば、コメントウィジェットに変更が必要になった際に、当社の自動化ツールで設定を検証できます。たとえば、ウィジェットカスタマイズ UI でカスタマイズルールを作成し、`Advanced` をクリックして `Custom CSS` に次のように入力します：

    @import url(https://example.com/styles.css);

#### コードでの使用 - 推奨しません

`customCSS` プロパティを使用して外部 CSS ファイルをロードすることもできます：

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

ただし、これを行うと当社が CSS をテストできなくなることに注意してください。

### ユーザープロファイルモーダルのスタイリング

ユーザープロファイルモーダルもカスタム CSS でスタイリングできます。ただし、ユーザープロファイルにカスタムスタイリングを適用するには、すべての CSS セレクタに `.user-profile` プレフィックスを付ける必要があります。このプレフィックスがないと、ユーザープロファイルモーダルのカスタムスタイリングは無視されます。

例：

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### 後方互換性

FastComments では、顧客がコメントウィジェットをカスタマイズすることを前提に設計されています。これは意図的なものであり、製品が顧客のデザインに不整合をもたらすことは望んでいません。

この重要な機能のため、当社はリリースごとにコメントウィジェットの変更を顧客ごとにレビューできるビルドパイプラインを備えています。

小さな問題が見つかった場合は、リリースが円滑に進むようにアカウントを更新します。重大な破壊的変更が見つかった場合は、リリースを中止できるようにしています。
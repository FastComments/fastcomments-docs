[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments はカスタマイズできるように設計されています。コメント用ウィジェット自体はセキュリティ上の理由から iframe 内で動作するため、カスタムスタイリングを適用するには次のいずれかの方法に従う必要があります。

最初で、最も簡単な方法（かつ当社推奨）は、[widget customization page](https://fastcomments.com/auth/my-account/customize-widget) を使用することです。

ウィジェットカスタマイゼーションページでは、"Show Advanced Options" セクションを参照し、その下に "Custom CSS" とラベル付けされた領域があります。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

この方法にはいくつかの利点があります:
1. 入力された CSS はユーザーに送信される前に圧縮され、編集 UI のフォーマットが一貫して保たれます。
2. ウィジェットカスタマイズ UI のすべての利点を享受できます。例えば、サイトごとにコメントウィジェットを簡単に異なるカスタマイズにできます。
3. コメントウィジェットに変更を加える際、あなたのカスタムスタイルは当社のリリースプロセスの一部としてテストされます。

2つ目の方法は、ウィジェット構成で **customCSS** パラメータを指定することです。以下のように:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

ただし、これは*制限*があります:
1. ヘッダーのサイズのため、サーバーがリクエストを拒否する前に渡せるカスタム CSS の量には制限があります。
2. カスタム CSS をあなたのインフラやビルドシステムで管理する必要があります。これはむしろ利点になる場合もあります。
3. このユースケースでは、カスタム CSS をネットワーク上で**2回**送信する追加のオーバーヘッドがあります。まず当社のサーバーへ送信され、次に iframe コンテンツ内で返送されるためです。ただし、ほとんどのペイロードサイズではこれは目立ちません。
4. 一般的な最適化として、ネットワーク上のサイズを削減するために CSS を圧縮（ミニファイ）することがありますが、この方法ではそれを自分で処理する必要があります。
5. この方法では、当社が変更を行った際にあなたのカスタム CSS はテストされません。

### External CSS Files

ウィジェットに外部ファイルをフェッチさせるには `@import` を使用できます！

`@import` をカスタマイズルールに入れておくことを推奨します。こうすることで、当社がコメントウィジェットに変更を加える必要が生じた場合に、当社の自動化ツールを使ってあなたの設定を検証できます。例えば、ウィジェットカスタマイズ UI でカスタマイズルールを作成し、`Advanced` をクリックして `Custom CSS` に次のように入力します:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

外部 CSS ファイルは `customCSS` プロパティ経由でも読み込めます:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

ただし、この方法を採ると当社であなたの CSS をテストできないことを忘れないでください。

### User Profile Modal Styling

ユーザープロファイルのモーダルもカスタム CSS でスタイリングできます。ただし、ユーザープロファイルにカスタムスタイリングを適用するには、すべての CSS セレクタが `.user-profile` でプレフィックスされている必要があります。このプレフィックスがない場合、ユーザープロファイルモーダルにはカスタムスタイリングは適用されません。

例えば:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

FastComments では、お客様がコメントウィジェットをカスタマイズすることを承知しています。それは設計上の意図です — 当社の製品があなたの製品のデザイン不整合を引き起こす最後のものにしたくありません。

これは当社の製品の重要な部分であるため、リリースごとに顧客ごとのコメントウィジェットの変更をレビューできるビルドパイプラインを用意しています。

軽微な問題が見つかった場合は、リリースが円滑に進むようにお客様のアカウントを更新します。重大な破壊的変更が見られる場合は、リリースを停止することができます。
[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

コメントスレッドを表示したり、コメントを投稿したりする際、FastComments はそのコメントがどのページ、記事、または製品に属しているかを把握する必要があります。

これを行うために、私たちは「URL ID」と呼んでいるものを使用します。これは文字列や数字のような識別子、または URL です。

デフォルトでは、urlId を指定しない場合、ページの URL が URL ID になります。現在のページの URL を取得し、一般的なマーケティングパラメータやトラッキング識別子を削除してクリーンにします。

WordPress のようなサードパーティ統合の場合、プラグインは通常、表示中の情報を表す識別子（たとえば記事/ページ ID）を URL ID として使用します。

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

本ドキュメントでよく参照するものの一つに、<a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget Customization UI</a> があります。

この UI を使うと、コードを使わずにコメントウィジェットを多くの点で変更できます。

カスタマイズルールを作成する際、サイトのすべてのページに適用したいことがよくあります。しかし、特定のページに対してコメントウィジェットをカスタマイズしたい場合もあります。例えば、特定のページだけカスタムスタイリングを適用したり、そのページのコメントを匿名にしたりすることができます。また、あるページではライブコメントをすぐに表示し、別のページでは通知ボタンの下に隠す、といったことも可能です。

これはすべて、このページの URL ID 入力フィールドを使って実現できます。見た目は次のようになっています:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

このフィールドの値は、コメントウィジェットに渡される *urlId* パラメータと一致している必要があります。カスタマイズルールを *urlId* 非依存にしたい場合は、このフィールドを空にするか * を入力してください。

As of 2023 the `URL ID` field in widget customization now also takes patterns! For example you may
have `*/blog/*` to add styling specific to your blog and `*/store/*` to have styling specific to your store,
all while using the same domain.

### 注意点

1. ページにハッシュパラメータ（例: example.com#page-1）がある場合 - デフォルトではこれが URL ID の一部になります。
2. WordPress から Gatsby への移行などのマイグレーション中は、初回の移行後に URL ID コメント値を移行する必要がある場合があります。その際はお問い合わせください。
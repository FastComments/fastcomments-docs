[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

通知メールの送信時や、モデレーションページなどのユーザーインターフェースにコメントを表示する際に、コメントからそのコメントが存在するページへのリンクを張れると便利です。

URL ID が常に ID であるとは限らない場合、URL を別の場所に保存する必要があります。それが "url" プロパティの役割で、以下のように定義されています。

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'カスタム URL の定義'; code-example-end]

よくあるユースケースは、コメントスレッドを記事などの識別子に紐付け、その上で特定のページにリンクを張ることです。例えば:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'カスタム URL と URL ID の同時定義'; code-example-end]

URL から一般的なマーケティングパラメータは除去されません。デフォルトでは、現在のページの URL がそのままコメントと共に保存されます。
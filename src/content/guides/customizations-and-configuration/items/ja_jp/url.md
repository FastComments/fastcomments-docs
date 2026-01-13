[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

通知メールを送信したり、モデレーションページのようなユーザーインターフェースでコメントを表示したりする際に、コメントから
それが掲載されているページへリンクできると便利です。

If URL ID isn't always an ID, then we have to store the URL some place else. That's what the "url" property is for, defined as follows.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

一般的なユースケースは、コメントスレッドを記事のような識別子に紐付け、その後特定のページにリンクすることです。例えば：

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

The URL does not get cleaned of common marketing parameters. By default, whatever the current page URL is, is the URL stored with the comment.
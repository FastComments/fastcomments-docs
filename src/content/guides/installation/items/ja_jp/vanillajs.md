The VanillaJS バージョンのウィジェットは、ビルドシステムやサーバーサイドコードを必要とせずに、ウェブサイトにコメントを追加する最もシンプルな方法です。

Simply add the following code snippet to any page to add comments to your site:

[code-example-start config = {}; title = 'シンプルなコードスニペット'; code-example-end]

You can use the same code snippet on many pages; it will automatically create a separate thread per page.

Many applications have an "HTML Embed Code" option. Select that and paste the code snippet above in.

*You also don't need an account to try it!* You might see "tenantId: demo" in the above snippet if you're not logged in. This way it will use
the demo account.

You can find documentation on configuring the widget <a href="/guide-customizations-and-configuration.html" target="_blank">here</a>.

All versions of the FastComments widget are wrappers around the core VanillaJS library. This allows us to add features
and fix issues in one place - and the changes automatically propagate to the other variants of the commenting widget.
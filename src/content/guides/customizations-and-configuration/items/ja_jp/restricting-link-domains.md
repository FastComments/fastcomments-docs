---
デフォルトでは、FastCommentsは任意の外部サイトへのリンクを許可します。

これは、代わりに指定したサイトやドメインのリストに制限できます。サイト、またはドメインにリンクを投稿しようとすると、
定義されたリストに含まれていない場合は、ユーザーにエラーが表示されます。

この検証はコメントウィジェットとAPIにのみ適用されます。インポートは影響を受けません。

これはコードを書くことなく、ウィジェットのカスタマイズページで行います：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.restricted-link-domains-list'; selector = '.external-link-settings'; title='Restrict External Link Domains' app-screenshot-end]
---
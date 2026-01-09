FastComments は Hostinger 向けの Page Reacts（別名フローティング「いいね」ボタン）ウィジェットにも対応しています。

このページの右下で実際の動作を確認できます！

### 注意！

これらの手順は Hostinger のサイトビルダー向けです。Hostinger の *WordPress* を使用している場合は、以下のコードを取得して、サイトに追加してください。小さなコードスニペットをサイトに追加するための無料で使いやすいプラグインである [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/) を使用します。

1. First, grab the code:

[inline-code-attrs-start title = 'Hostinger フローティングいいねコード'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Then, in Hostinger, open the site builder.
3. Go to Website Settings in the bottom Left.
4. Select Integrations.
5. Add the new code to the *end* of the `Custom code` field, and publish your site.
6. You will not see the widget in preview mode, but it will appear on the published version of the site.
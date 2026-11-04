FastCommentsは、Hostinger向けのPage Reacts(別名Floating Likeボタン)ウィジェットにも対応しています。

このページの右下で実際に動作している様子をご覧いただけます!

### 注意!

この手順はHostingerのサイトビルダー向けです。Hostingerの *WordPress* をご利用の場合は、以下のコードを取得し、[WPCode](https://wordpress.org/plugins/insert-headers-and-footers/) を使ってWordPressサイトに追加してください。WPCodeは、サイトに小さなコードスニペットを追加するための無料で簡単なプラグインです。

1. まず、コードを取得します:

[inline-code-attrs-start title = 'Hostinger Floating Likes コード'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. 次に、Hostingerでサイトビルダーを開きます。
3. 左下の「Website Settings」に移動します。
4. 「Integrations」を選択します。
5. 新しいコードを `Custom code` フィールドの *末尾* に追加し、サイトを公開します。
6. プレビューモードではウィジェットは表示されませんが、公開されたサイトには表示されます。
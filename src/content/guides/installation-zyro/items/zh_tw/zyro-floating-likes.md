FastComments 也支援 Zyro 的 Page Reacts（又名浮動按讚按鈕）小工具。

你可以在本頁的右下角看到它的作用！

1. 首先，取得程式碼：

[inline-code-attrs-start title = 'Zyro 浮動按讚程式碼'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. 然後，在 Zyro 中，打開網站建置工具。
3. 前往左下角的網站設定。
4. 選擇整合。
5. 將新的程式碼新增到 `Custom code` 欄位的*結尾*，然後發佈您的網站。
6. 您在預覽模式看不到此小工具，但在已發佈的網站版本上會顯示。

---
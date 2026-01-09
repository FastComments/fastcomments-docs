---
FastComments 也支援 Hostinger 的 Page Reacts（又稱浮動按讚按鈕）小工具。

你可以在本頁的右下角看到其運作！

### 注意！

這些說明適用於 Hostinger 網站建構器。如果你使用的是 Hostinger *WordPress*，只要取得下面的程式碼，並使用 [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/)（一個免費且簡單的外掛，用於在網站中加入小段程式碼）將其加入你的 WordPress 網站即可。

1. 首先，取得程式碼：

[inline-code-attrs-start title = 'Hostinger 浮動按讚程式碼'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. 接著，在 Hostinger 中打開網站建構器。
3. 前往左下角的網站設定。
4. 選擇整合。
5. 將新程式碼新增到 *末端* 的 `Custom code` 欄位，並發佈你的網站。
6. 在預覽模式中你將看不到該小工具，但在發佈後的網站版本中它會顯示出來。

---
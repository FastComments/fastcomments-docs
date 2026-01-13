建議不要透過 BigCommerce 的頁面建構器新增 FastComments，因為這樣程式碼必須手動加入到每一個想要的頁面。

然而，如果仍希望這麼做，必須使用下列程式碼片段。由於 BigCommerce 的特性，其他教學中的程式碼片段將無法運作：

[inline-code-attrs-start title = 'BigCommerce 頁面建構器 片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        function attemptLoad() {
            if (loaded) {
                return;
            }
            if (!window.FastCommentsUI) {
                return;
            }
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo"
            });
            loaded = true;
        }
        attemptLoad();
        const interval = setInterval(function () {
            attemptLoad();
            if (loaded) {
                clearInterval(interval);
            }
        }, 300);
    })();
</script>
[inline-code-end]

---
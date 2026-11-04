現在我們可以複製以下程式碼片段(使用片段右上角的複製按鈕):

[inline-code-attrs-start title = 'Squarespace 部落格留言程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // your account id

        function tryLoad() {
            // try to load for different layouts
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...然後貼到程式碼區域並點擊儲存:

<div class="screenshot white-bg">
    <div class="title">貼上並儲存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="貼上並儲存" />
</div>
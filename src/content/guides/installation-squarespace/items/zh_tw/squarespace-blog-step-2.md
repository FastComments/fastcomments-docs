Now we can copy the following code snippet (use the copy button in the top right of the snippet):

[inline-code-attrs-start title = 'Squarespace 部落格評論程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // 你的帳戶 ID

        function tryLoad() {
            // 嘗試為不同佈局載入
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

...then paste in the code area and click save:

<div class="screenshot white-bg">
    <div class="title">貼上並儲存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="貼上並儲存" />
</div>

---
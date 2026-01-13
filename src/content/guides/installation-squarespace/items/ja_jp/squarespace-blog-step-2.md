---
次に、以下のコードスニペットをコピーできます（スニペット右上のコピー ボタンを使用してください）:

[inline-code-attrs-start title = 'Squarespace ブログのコメントコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // あなたのアカウントID

        function tryLoad() {
            // 様々なレイアウトで読み込もうとします
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

...次に、コード領域に貼り付けて保存をクリックします:

<div class="screenshot white-bg">
    <div class="title">貼り付けて保存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="貼り付けて保存" />
</div>

---
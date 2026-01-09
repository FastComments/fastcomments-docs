现在我们可以复制以下代码片段（使用片段右上角的复制按钮）：

[inline-code-attrs-start title = 'Squarespace 博客评论代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // 你的账户 ID

        function tryLoad() {
            // 尝试为不同布局加载
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

...然后将其粘贴到代码区域并点击保存：

<div class="screenshot white-bg">
    <div class="title">粘贴并保存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="粘贴并保存" />
</div>
---
现在我们要复制代码。如果代码片段在第6行显示 `tenantId: "demo"`，那么你应该登录你的 FastComments 帐户
然后刷新此页面，以便复制的代码片段包含你的帐户 ID。

[inline-code-attrs-start title = 'Systeme.io 代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

现在将其粘贴到编辑器中并点击保存：

<div class="screenshot white-bg">
    <div class="title">添加 FastComments 代码</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="添加 FastComments 代码" />
</div>

... 然后保存你的网站。就这样！

---
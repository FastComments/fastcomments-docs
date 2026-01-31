不建议通过 BigCommerce 的页面构建器添加 FastComments，因为这样必须将代码手动添加到每个目标页面。

但是，如果确实需要这样做，则必须使用以下代码片段。由于 BigCommerce 的特性，其他教程中的代码片段将无法正常工作：

[inline-code-attrs-start title = 'BigCommerce 页面构建器片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

---
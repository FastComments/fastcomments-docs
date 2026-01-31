不建議透過 BigCommerce 的 Page Builder 新增 FastComments，因為這樣必須手動將程式碼加入每個想要的頁面。

然而，如果真的想這樣做，必須使用下列程式碼片段。由於 BigCommerce 的特性，其他教學中的程式碼片段將無法使用：

[inline-code-attrs-start title = 'BigCommerce Page Builder 程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
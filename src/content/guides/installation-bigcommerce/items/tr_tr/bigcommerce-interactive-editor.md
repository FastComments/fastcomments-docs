FastComments'ı BigCommerce'in Page Builder'ı üzerinden eklemek önerilmez; çünkü kodun her istenen sayfaya elle eklenmesi gerekir.

Ancak, eğer bu isteniyorsa, aşağıdaki kod parçası kullanılmalıdır. BigCommerce'in yapısı gereği diğer eğitimlerdeki kod parçacıkları çalışmayacaktır:

[inline-code-attrs-start title = 'BigCommerce Page Builder Kod Parçası'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
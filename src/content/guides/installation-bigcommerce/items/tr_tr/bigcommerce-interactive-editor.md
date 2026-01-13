FastComments'ı BigCommerce'in Sayfa Oluşturucusu aracılığıyla eklemek önerilmez çünkü bu durumda kodun her istenen sayfaya manuel olarak eklenmesi gerekir.

Ancak, eğer bu isteniyorsa, aşağıdaki kod parçacığı kullanılmalıdır. BigCommerce'in doğası gereği diğer öğreticilerdeki kod parçacıkları çalışmayacaktır:

[inline-code-attrs-start title = 'BigCommerce Sayfa Oluşturucu Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
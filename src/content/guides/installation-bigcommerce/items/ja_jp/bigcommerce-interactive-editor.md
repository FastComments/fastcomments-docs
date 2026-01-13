---
FastComments を BigCommerce の Page Builder 経由で追加することは推奨されません。そうするとコードを目的の各ページに手動で追加する必要があるためです。

ただし、それでも追加したい場合は、次のコードスニペットを使用する必要があります。他のチュートリアルのコードスニペットは、BigCommerce の性質上動作しません:

[inline-code-attrs-start title = 'BigCommerce の Page Builder スニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
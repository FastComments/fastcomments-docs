Nie zaleca się dodawania FastComments za pomocą kreatora stron (Page Builder) BigCommerce, ponieważ wtedy kod musi być ręcznie dodawany na każdej wybranej stronie.

Jeśli jednak jest to pożądane, należy użyć następującego fragmentu kodu. Fragmenty kodu z innych samouczków nie będą działać ze względu na specyfikę BigCommerce:

[inline-code-attrs-start title = 'Fragment kreatora stron BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
Ne priporočamo dodajanja FastComments prek BigCommerce's Page Builder, saj je v tem primeru treba kodo ročno dodati na vsako želeno stran.

Če pa je to zaželeno, je treba uporabiti naslednji del kode. Delčki kode iz drugih vadnic ne bodo delovali zaradi narave BigCommerce:

[inline-code-attrs-start title = 'Vstavek za BigCommerce Page Builder'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
---
Nije preporučljivo dodavati FastComments putem BigCommerce-ovog Page Builder-a, jer tada kod mora biti ručno dodat na svaku željenu stranicu.

Međutim, ako je to poželjno, mora se koristiti sledeći isječak koda. Isječci koda iz drugih tutorijala neće raditi zbog prirode BigCommerce-a:

[inline-code-attrs-start title = 'Isječak za BigCommerce Page Builder'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
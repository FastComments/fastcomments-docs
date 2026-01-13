---
Не препоручује се додавање FastComments-а преко BigCommerce-овог Page Builder-а, јер тада код мора бити ручно додат на сваку жељену страницу.

Међутим, ако је ово жељено, мора се користити следећи исечак кода. Исечци кода из других туторијала неће радити због природе BigCommerce-а:

[inline-code-attrs-start title = 'Исечак Page Builder-а за BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
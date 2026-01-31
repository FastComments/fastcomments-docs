Не препоручује се додавање FastComments-а преко BigCommerce Page Builder-а, јер тада код мора бити ручно додат на сваку страницу на којој желите.

Међутим, ако је то жељено, мора се користити следећи фрагмент кода. Фрагменти кода из других туторијала неће радити због природе BigCommerce-а:

[inline-code-attrs-start title = 'Фрагмент кода за BigCommerce Page Builder'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
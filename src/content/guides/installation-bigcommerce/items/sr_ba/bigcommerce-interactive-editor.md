Није препоручљиво додавање FastComments-а преко BigCommerce-овог Page Builder-а јер у том случају код мора бити ручно додан на сваку страницу на којој желите.

Међутим, ако је ово жељено, мора се користити следећи исечак кода. Исечци кода из других упутстава неће радити због природе BigCommerce-а:

[inline-code-attrs-start title = 'Исечак за BigCommerce Page Builder'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
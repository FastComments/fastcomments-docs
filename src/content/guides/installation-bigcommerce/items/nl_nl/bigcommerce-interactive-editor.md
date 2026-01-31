---
Het wordt niet aanbevolen om FastComments toe te voegen via de Page Builder van BigCommerce, omdat de code dan handmatig aan elke gewenste pagina moet worden toegevoegd.

Als dit echter gewenst is, moet het volgende codefragment worden gebruikt. Codefragmenten uit andere tutorials werken niet vanwege de aard van BigCommerce:

[inline-code-attrs-start title = 'BigCommerce Page Builder-codefragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
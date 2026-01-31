---
Det anbefales ikke at tilføje FastComments via BigCommerce's Page Builder, da koden så skal tilføjes manuelt til hver ønskede side.

Hvis dette ønskes, skal følgende kodeudsnit bruges. Kodeudsnit fra andre vejledninger vil ikke fungere på grund af måden BigCommerce er opbygget på:

[inline-code-attrs-start title = 'BigCommerce Page Builder-kodeudsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
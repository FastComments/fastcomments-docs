---
Fragmenti koda i biblioteke za front-end za On-Prem su isti kao kod SaaS proizvoda. Međutim, morate navesti `apiHost` i ispravan put do skripta:

[inline-code-attrs-start title = 'Kod komentara za On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... možete takođe proslediti SSO payload itd.
    }];
</script>
[inline-code-end]

Gornji primer je veoma jednostavan. Takođe možemo koristiti zvanične React, Angular, Vue, Svelte itd. biblioteke.

---
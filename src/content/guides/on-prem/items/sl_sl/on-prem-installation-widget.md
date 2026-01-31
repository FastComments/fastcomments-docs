---
Koda vmesnika (front-end) in knjižnice za On-Prem so enake kot pri izdelku SaaS. Vendar pa morate navesti `apiHost` in pravilno pot do skripte:

[inline-code-attrs-start title = 'Koda komentarjev za On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... lahko tudi posredujete SSO podatke itd.
    }];
</script>
[inline-code-end]

Zgornji primer je zelo preprost. Uporabili bi lahko tudi uradne knjižnice za React, Angular, Vue, Svelte itd.

---
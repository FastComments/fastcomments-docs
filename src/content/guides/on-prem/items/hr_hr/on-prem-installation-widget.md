---
Fragmenti koda i biblioteke za front-end za On-Prem su isti kao i za SaaS proizvod. Međutim, morate navesti `apiHost` i ispravnu putanju skripte:

[inline-code-attrs-start title = 'Kod komentara za On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... također možete proslijediti SSO payload itd.
    }];
</script>
[inline-code-end]

Gore je vrlo jednostavan primjer. Također možemo koristiti 1st-party biblioteke za React, Angular, Vue, Svelte itd.

---
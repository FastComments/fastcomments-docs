---
Front-end isječci koda i biblioteke za On-Prem su isti kao i za SaaS proizvod. Međutim, morate navesti `apiHost` i ispravnu putanju skripte:

[inline-code-attrs-start title = 'Kod komentara za On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... can also pass SSO payload etc.
    });
</script>
[inline-code-end]

Gore je vrlo jednostavan primjer. Također možemo koristiti službene biblioteke za React, Angular, Vue, Svelte itd.

---
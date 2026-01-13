Koda za front-end (odjemalska stran) in knjižnice za On-Prem so enake kot pri izdelku SaaS. Vendar morate navesti `apiHost` in pravilno pot do skripte:

[inline-code-attrs-start title = 'Koda komentarjev za On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... lahko tudi posredujete SSO podatke itd.
    });
</script>
[inline-code-end]

Zgoraj je zelo preprost primer. Uporabimo lahko tudi uradne knjižnice za React, Angular, Vue, Svelte itd.

---
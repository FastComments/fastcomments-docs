I frammenti di codice e le librerie per il front end per On-Prem sono gli stessi del prodotto SaaS. Tuttavia, è necessario specificare `apiHost` e il percorso dello script corretto:

[inline-code-attrs-start title = 'Codice dei commenti per On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... è possibile anche passare il payload SSO ecc.
    }];
</script>
[inline-code-end]

Quanto sopra è un esempio molto semplice. Potremmo anche usare le librerie ufficiali 1st-party per React, Angular, Vue, Svelte, ecc.
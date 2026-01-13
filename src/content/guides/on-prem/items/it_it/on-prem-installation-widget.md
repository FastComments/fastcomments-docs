---
I frammenti di codice front-end e le librerie per On-Prem sono gli stessi del prodotto SaaS. Tuttavia, è necessario specificare `apiHost` e il percorso dello script corretto:

[inline-code-attrs-start title = 'Codice dei commenti per On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... si può anche passare il payload SSO, ecc.
    });
</script>
[inline-code-end]

L'esempio sopra è molto semplice. Possiamo anche usare le librerie di prima parte per React, Angular, Vue, Svelte, ecc.

---
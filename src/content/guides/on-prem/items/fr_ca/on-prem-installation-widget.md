Les extraits de code front-end et les bibliothèques pour On-Prem sont les mêmes que pour le produit SaaS. Cependant, vous devez spécifier `apiHost` et le chemin de script correct :

[inline-code-attrs-start title = 'Code des commentaires pour On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... peut aussi passer une charge utile SSO, etc.
    });
</script>
[inline-code-end]

Ce qui précède est un exemple très simple. Nous pourrions aussi utiliser les bibliothèques 1st-party React, Angular, Vue, Svelte, etc.

---
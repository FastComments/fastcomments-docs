---
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

L'exemple ci‑dessus est très simple. Nous pourrions également utiliser les bibliothèques officielles pour React, Angular, Vue, Svelte, etc.

---
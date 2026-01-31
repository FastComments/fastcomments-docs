Les extraits de code et les bibliothèques front-end pour On-Prem sont les mêmes que pour le produit SaaS. Cependant, vous devez spécifier `apiHost` et le chemin de script correct :

[inline-code-attrs-start title = 'Code des commentaires pour On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... peut également transmettre la charge utile SSO, etc.
    }];
</script>
[inline-code-end]

Ce qui précède est un exemple très simple. Nous pouvons également utiliser les bibliothèques officielles React, Angular, Vue, Svelte, etc.
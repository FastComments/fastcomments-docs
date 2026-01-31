---
Los fragmentos de código y las bibliotecas del front-end para On-Prem son los mismos que los del producto SaaS. Sin embargo, debe especificar `apiHost` y la ruta correcta del script:

[inline-code-attrs-start title = 'Código de comentarios para On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... también se puede pasar la carga útil de SSO, etc.
    }];
</script>
[inline-code-end]

Lo anterior es un ejemplo muy simple. También podríamos usar las bibliotecas oficiales o de primera parte para React, Angular, Vue, Svelte, etc.

---
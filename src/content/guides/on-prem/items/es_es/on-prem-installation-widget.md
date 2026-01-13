Los fragmentos de código de front-end y las bibliotecas para On-Prem son los mismos que los del producto SaaS. Sin embargo, debe especificar `apiHost` y la ruta de script correcta:

[inline-code-attrs-start title = 'Código de comentarios para On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... también puede pasar la carga útil SSO, etc.
    });
</script>
[inline-code-end]

Lo anterior es un ejemplo muy simple. También podríamos usar las bibliotecas oficiales de React, Angular, Vue, Svelte, etc.
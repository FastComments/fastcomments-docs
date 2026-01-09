Os trechos de código e bibliotecas de front-end para On-Prem são os mesmos do produto SaaS. No entanto, você deve especificar `apiHost` e o caminho correto do script:

[inline-code-attrs-start title = 'Código de Comentários para On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... também é possível passar o payload de SSO, etc.
    });
</script>
[inline-code-end]

O exemplo acima é muito simples. Também podemos usar as bibliotecas oficiais para React, Angular, Vue, Svelte, etc, libraries.

---
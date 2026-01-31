Os trechos de código do front-end e as bibliotecas para On-Prem são os mesmos do produto SaaS. No entanto, você deve especificar `apiHost` e o caminho correto do script:

[inline-code-attrs-start title = 'Código de Comentários para On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... também pode passar payload SSO, etc.
    }];
</script>
[inline-code-end]

O acima é um exemplo muito simples. Também podemos usar as bibliotecas oficiais de primeira parte para React, Angular, Vue, Svelte, etc.
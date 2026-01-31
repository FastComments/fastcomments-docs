Agora podemos copiar o seguinte trecho de código. Use o botão de copiar que aparece no canto superior direito do trecho.

Há algumas coisas que você pode configurar no código, veja as linhas 4 a 7.

[inline-code-attrs-start title = 'Código de Página Única do Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // o id da sua conta
    }];
</script>
[inline-code-end]

Deve ficar assim:

<div class="screenshot white-bg">
    <div class="title">Colar e Salvar</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Colar e Salvar" />
</div>

Agora clique em Salvar no canto superior direito.

Observe que a opção `Preview in Safe Mode` não funcionará, mas o widget aparecerá quando você visitar seu site.

Se estiver com problemas, verifique perto do final se não aparece `"tenantId": "demo"`. Deve mostrar o seu tenant id se você estiver logado. Caso contrário, entre em contato com o suporte.
Agora podemos copiar o trecho de código a seguir. Use o botão de copiar que aparece no canto superior direito do trecho.

Há algumas coisas que você pode configurar no código, veja as linhas 4 a 7.

[inline-code-attrs-start title = 'Código de Comentários para Todas as Páginas do Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // seu ID da conta

        function tryLoad() {
            // tenta carregar para diferentes layouts
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...então cole na área de código e clique em salvar. Deve ficar assim, com o código no bloco `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Cole e Salve</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Cole e Salve" />
</div>

Se você estiver tendo problemas, verifique se, próximo ao final, não diz `"tenantId": "demo"`. Ele deve mostrar seu tenant id se você estiver logado. Caso contrário, entre em contato com o suporte.
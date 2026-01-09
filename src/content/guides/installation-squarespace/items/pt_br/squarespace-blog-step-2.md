---
Agora podemos copiar o trecho de código a seguir (use o botão de copiar no canto superior direito do trecho):

[inline-code-attrs-start title = 'Código de Comentários do Blog Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // seu id da conta

        function tryLoad() {
            // tentar carregar para diferentes layouts
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

...em seguida cole na área de código e clique em salvar:

<div class="screenshot white-bg">
    <div class="title">Colar e Salvar</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Colar e Salvar" />
</div>

---
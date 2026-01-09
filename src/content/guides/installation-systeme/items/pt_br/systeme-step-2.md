---
Agora vamos copiar nosso código. Se o trecho de código mostrar `tenantId: "demo"` na linha 6 então você deve entrar na sua conta FastComments e depois atualizar esta página para que o trecho de código copiado contenha o id da sua conta.

[inline-code-attrs-start title = 'Snippet do Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Agora cole-o no editor e clique em salvar:

<div class="screenshot white-bg">
    <div class="title">Adicione o código do FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Adicione o código do FastComments" />
</div>

... então salve seu site. É isso!

---
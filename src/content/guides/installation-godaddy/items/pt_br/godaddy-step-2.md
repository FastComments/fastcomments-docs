Agora que você adicionou um bloco HTML personalizado, podemos adicionar o código do widget do FastComments.

**Use o código a seguir para Godaddy, não o código de outros tutoriais. Este código é específico para Godaddy.**

Copie o código a seguir:

[inline-code-attrs-start title = 'Trecho de Código de Comentários do Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Esse trecho de código específico foi projetado para ser compatível com o Godaddy, e também só será exibido nas suas postagens do blog - não na página inicial.

Agora cole o código na área `Custom Code` mencionada em `Step One`.

<div class="screenshot white-bg">
    <div class="title">Copie e Cole o Código</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Copie e Cole o Código" />
</div>

Clique em Done no canto superior direito:

<div class="screenshot white-bg">
    <div class="title">Clique em Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Clique em Done" />
</div>

Isso é tudo para a etapa dois!
FastComments também suporta o widget Page Reacts (também conhecido como botão de Curtir flutuante) para Zyro.

Você pode vê-lo em ação no canto inferior direito desta página!

1. Primeiro, pegue o código:

[inline-code-attrs-start title = 'Código de Curtidas Flutuantes do Zyro'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Em seguida, no Zyro, abra o construtor de sites.
3. Vá para Configurações do Site no canto inferior esquerdo.
4. Selecione Integrações.
5. Adicione o novo código ao *final* do campo `Custom code`, e publique seu site.
6. Você não verá o widget no modo de visualização, mas ele aparecerá na versão publicada do site.

---
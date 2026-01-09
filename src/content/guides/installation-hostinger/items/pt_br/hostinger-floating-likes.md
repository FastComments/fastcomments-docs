---
FastComments também oferece suporte ao widget Page Reacts (também conhecido como botão Flutuante de Curtida) para Hostinger.

Você pode vê-lo em ação no canto inferior direito desta página!

### Nota!

Estas instruções são para o Hostinger Site Builder. Se você estiver usando Hostinger *WordPress*, simplesmente copie o código abaixo e adicione-o ao seu site WordPress
usando [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), que é um plugin gratuito e fácil para adicionar pequenos trechos de código ao seu site.

1. Primeiro, copie o código:

[inline-code-attrs-start title = 'Código de Curtidas Flutuantes do Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Em seguida, no Hostinger, abra o construtor de sites.
3. Vá para Configurações do Site no canto inferior esquerdo.
4. Selecione Integrações.
5. Adicione o novo código ao *final* do campo `Custom code`, e publique seu site.
6. Você não verá o widget no modo de visualização, mas ele aparecerá na versão publicada do site.

---
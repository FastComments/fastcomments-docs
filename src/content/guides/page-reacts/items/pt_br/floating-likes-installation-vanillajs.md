A instalação é simples:

[inline-code-attrs-start title = 'Exemplo de código Curtidas Flutuantes'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

A assinatura de tipo do construtor é:

[inline-code-attrs-start title = 'Configuração'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - Altere isto para definir o id da página/artigo. Por padrão, é a URL da página.
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

Ele suporta SSO para vincular as reações à conta do usuário para autenticação.

Atualmente, apenas VanillaJS é suportado. Se você gostaria que esse widget fosse adicionado a uma de nossas bibliotecas cliente, avise-nos! 

### Versão Assíncrona

[inline-code-attrs-start title = 'Exemplo de código Curtidas Flutuantes Assíncrono'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (window.FastCommentsEmbedPageLikesFloating) {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: 'demo'
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

### Com SSO

Também podemos passar payloads de Secure SSO ou Simple SSO:

[inline-code-attrs-start title = 'Exemplo de código Curtidas Flutuantes SSO Seguro'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // passe o objeto sso
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = 'Exemplo de código Curtidas Flutuantes SSO Simples'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        simpleSSO: {
            id: 'some-user-id',
            username: 'some username',
            email: 'some@email.com',
        }
    });
</script>
[inline-code-end]

---
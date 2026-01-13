Para usar o FastComments SSR, o cliente pode buscar HTML do endpoint `https://fastcomments.com/ssr/comments`.

Isso pode ser feito de várias maneiras.

### Com WordPress

O SSR é ativado por padrão para usuários sem JS habilitado como um fallback no plugin do WordPress desde a versão `3.10.2`.

### Em uma página da Web

Em uma aplicação já existente, o SSR pode ser adicionado com o [exemplo a seguir](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), assumindo que a linguagem usada seja PHP:

[inline-code-attrs-start title = 'Exemplo de SSR baseado em PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

Também podemos mostrar a UI do SSR somente quando o usuário tiver o JS desabilitado:

[inline-code-attrs-start title = 'Exemplo de fallback de SSR baseado em PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

Para um exemplo usando SSO, [veja aqui](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Com Conteúdo Pré-renderizado

Nosso blog é gerado em tempo de build e fornece um [bom exemplo de SSR com Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Os Parâmetros Básicos

Os parâmetros básicos que você precisa passar são:
- `tenantId` - Isso identifica você como cliente.
- `urlId` - Isso identifica a página ou artigo para carregar comentários e define onde eles são salvos.
- `url` - Isso é usado para notificações e recursos relacionados que vinculam de volta ao tópico de comentários.

### Estilização Personalizada

A versão SSR do widget de comentários usa a mesma estrutura e motor de renderização que a versão em JavaScript.

Assim, toda estilização personalizada que funciona para o widget de comentários em JavaScript funciona para o SSR. 

### Observações

Com SSR, não há JavaScript para controlar a altura do contêiner renderizado. Nos navegadores, uma barra de rolagem vertical pode aparecer para discussões longas.

Portanto, você deve ajustar isso conforme desejado.

---
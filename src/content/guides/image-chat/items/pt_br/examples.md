### Exemplo Básico

A maneira mais simples de usar o Image Chat é direcionar um único elemento de imagem. Este exemplo mostra como ativar discussões interativas em uma imagem:

[inline-code-attrs-start title = 'Exemplo básico de Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### Exemplo com elemento contêiner

Você também pode passar um elemento contêiner que tenha uma imagem dentro dele:

[inline-code-attrs-start title = 'Image Chat com contêiner'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### Exemplo com ID de URL personalizado

Por padrão, o Image Chat usa a URL da página combinada com a origem da imagem e as coordenadas para identificar conversas. Você pode fornecer um `urlId` personalizado:

[inline-code-attrs-start title = 'Image Chat com ID de URL personalizado'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Isto é útil se a estrutura da sua URL mudar, mas você quiser manter as mesmas conversas, ou se você quiser compartilhar os mesmos pontos de discussão em várias páginas.

### Exemplo com Modo Escuro

Se o seu site tiver um fundo escuro e o widget não estiver detectando automaticamente como deveria, podemos ativar manualmente o suporte ao modo escuro:

[inline-code-attrs-start title = 'Image Chat com Modo Escuro'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Exemplo com Tamanho Personalizado do Quadrado de Chat

Você pode ajustar o tamanho dos quadrados clicáveis que aparecem na imagem. O tamanho é especificado como uma porcentagem da largura da imagem:

[inline-code-attrs-start title = 'Image Chat com Tamanho de Quadrado Personalizado'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // Quadrados menores (padrão é 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Exemplo com Callback de Contagem de Comentários

Acompanhe quando comentários são adicionados ou atualizados usando o callback `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat com Callback de Contagem de Comentários'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### Exemplo com Múltiplas Imagens

Você pode inicializar o Image Chat em várias imagens. Cada imagem terá seus próprios pontos de discussão independentes:

[inline-code-attrs-start title = 'Image Chat em Múltiplas Imagens'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Inicializa na primeira imagem
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Inicializa na segunda imagem
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---
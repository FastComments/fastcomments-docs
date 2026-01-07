O Widget de Contagem de Comentarios em Massa e projetado para exibir eficientemente contagens de comentarios para multiplas paginas na mesma pagina. Em vez de fazer chamadas de API individuais para cada contagem de comentarios, este widget agrupa as solicitacoes para desempenho otimo.

## Instalacao Basica

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Como Funciona

O widget em massa funciona:

1. Escaneando a pagina por elementos com a classe `fast-comments-count`
2. Lendo o atributo `data-fast-comments-url-id` de cada elemento
3. Agrupando solicitacoes de API para buscar multiplas contagens de comentarios eficientemente
4. Atualizando cada elemento com a contagem de comentarios apropriada

## Opcoes de Configuracao

A funcao `FastCommentsCommentCountBulk` aceita as seguintes opcoes de configuracao:

- **tenantId** (obrigatorio): Seu ID de tenant do FastComments
- **apiHost** (opcional): Host de API personalizado se voce estiver usando uma instancia auto-hospedada

## Exemplo do Mundo Real

Aqui esta um exemplo pratico mostrando como voce pode usar o widget em massa em uma listagem de posts de blog:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Consideracoes de Desempenho

O widget em massa otimiza automaticamente o desempenho atraves de:

- **Agrupamento de solicitacoes**: Multiplas contagens de comentarios sao buscadas em uma unica chamada de API
- **Limites de tamanho de solicitacao**: Solicitacoes sao automaticamente divididas se a lista de URLs ficar muito grande (mais de 1.000 caracteres)
- **Deduplicacao**: Multiplos elementos com o mesmo `data-fast-comments-url-id` compartilham a mesma contagem

## Multiplos Elementos com o Mesmo URL ID

Voce pode ter multiplos elementos na pagina com o mesmo `data-fast-comments-url-id`. Todos serao atualizados com a mesma contagem:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Localizacao

O widget em massa formata automaticamente contagens de comentarios com base nas suas configuracoes de idioma do FastComments. Ele fornece texto apropriado para:

- Zero comentarios
- Um comentario
- Multiplos comentarios

## Quando Usar o Widget em Massa vs Individual

**Use o Widget em Massa quando:**
- Voce tem multiplas contagens de comentarios na mesma pagina
- Voce esta exibindo uma lista de posts/artigos com contagens de comentarios
- O desempenho e importante (reduz chamadas de API)

**Use o Widget Individual quando:**
- Voce precisa de apenas uma contagem de comentarios na pagina
- Voce precisa de atualizacoes ao vivo (o widget individual suporta atualizacoes em tempo real)
- Voce quer mais controle sobre o comportamento individual do widget

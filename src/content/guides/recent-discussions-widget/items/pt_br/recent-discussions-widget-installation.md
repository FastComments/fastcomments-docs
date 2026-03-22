O Widget Discussões Recentes mostra as páginas do seu site que têm a atividade de comentários mais recente. Cada entrada exibe o título da página, a data da última atividade e a contagem total de comentários. Ele detecta automaticamente fundos escuros e ajusta seu estilo de acordo.

## Instalação Básica

[inline-code-attrs-start title = 'Instalação do Widget de Discussões Recentes'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Opções de Configuração

A função `FastCommentsRecentDiscussionsV2` aceita as seguintes opções de configuração:

- **tenantId** (required): Seu ID de tenant do FastComments
- **count** (optional): Número de páginas a exibir. O padrão é `20`, máximo `100`
- **hasDarkBackground** (optional): Força o estilo do modo escuro. Detectado automaticamente a partir do fundo da página se não definido

## Exemplos Avançados

### Contagem Personalizada

[inline-code-attrs-start title = 'Discussões Recentes com Contagem Personalizada'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### Forçar Modo Escuro

[inline-code-attrs-start title = 'Discussões Recentes com Modo Escuro'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

---
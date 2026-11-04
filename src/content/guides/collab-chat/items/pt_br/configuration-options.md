### Visão Geral

O FastComments Collab Chat estende o widget de comentários padrão do FastComments, portanto herda todas as opções de configuração do widget base, além de adicionar algumas específicas para anotações de texto.

### Configuração Obrigatória

#### tenantId

O ID de Tenant do FastComments é obrigatório. Você pode encontrá-lo no seu [painel do FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Opções Específicas do Collab Chat

#### urlId

Por padrão, o Collab Chat gera um identificador único para cada conversa com base na URL da página, no caminho do DOM até o elemento e no intervalo de texto selecionado. Você pode sobrescrever isso com um `urlId` personalizado.

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Isso é útil quando a estrutura da sua URL pode mudar, mas você deseja manter as mesmas conversas, ou quando deseja compartilhar anotações entre várias páginas.

#### topBarTarget

Controla a exibição da barra superior, que mostra a contagem de usuários e de discussões. Defina como `null` para desativar completamente a barra superior, ou forneça um elemento DOM para renderizá-la em um local específico.

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Ative o estilo de modo escuro quando sua página tiver um fundo escuro. Essa detecção é automática, mas pode ser desejável sobrescrevê-la.

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Uma função de callback que é acionada sempre que a contagem de comentários muda. Isso é útil para atualizar elementos da interface, como badges ou títulos de página.

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Opções de Configuração Herdadas

Como o Collab Chat estende o widget de comentários padrão, você pode usar qualquer opção de configuração do widget base do FastComments. Aqui estão algumas opções comumente usadas:

#### locale

Define o idioma da interface do widget. O FastComments suporta dezenas de idiomas.

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Torna todas as conversas somente leitura. Os usuários podem visualizar anotações existentes, mas não podem criar novas nem responder.

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso e simpleSSO

Integre com seu sistema de autenticação usando Single Sign-On.

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Consulte a documentação de SSO para obter detalhes completos sobre as opções de autenticação.

#### maxReplyDepth

Controla quantos níveis de profundidade as respostas podem ter. Por padrão, o Collab Chat define isso como 0, o que significa que todos os comentários são planos (sem respostas aninhadas). Você pode alterar isso se quiser conversas encadeadas.

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Configuração Interna

Essas opções são definidas automaticamente pelo Collab Chat e não devem ser sobrescritas:

O `productId` é definido automaticamente como `3` para o Collab Chat. A extensão `floating-chat` é carregada automaticamente para fornecer a funcionalidade da janela de chat. O widget detecta automaticamente dispositivos móveis (telas com menos de 768px de largura) e ajusta a interface de acordo.

### Exemplo Completo

Aqui está um exemplo mostrando várias opções de configuração juntas:

[inline-code-attrs-start title = "Exemplo de Configuração"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

Para obter uma lista completa de todas as opções de configuração disponíveis herdadas do widget base, consulte a documentação principal de configuração do FastComments.
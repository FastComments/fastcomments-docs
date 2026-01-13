### Visão geral

FastComments Image Chat estende o widget de comentários padrão do FastComments, herdando todas as opções de configuração do widget base e adicionando algumas específicas para anotações em imagens.

### Configuração necessária

#### tenantId

Seu Tenant ID do FastComments é obrigatório. Você pode encontrá-lo no [painel do FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Opções específicas do Image Chat

#### urlId

Por padrão, o Image Chat gera um identificador único para cada conversa com base na URL da página, na origem da imagem e nas coordenadas X/Y. Você pode sobrescrever isso com um `urlId` personalizado.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Isso é útil quando a estrutura da sua URL pode mudar, mas você quer manter as mesmas conversas, ou quando deseja compartilhar anotações entre várias páginas.

#### chatSquarePercentage

Controla o tamanho dos marcadores de chat clicáveis como porcentagem da largura da imagem. O padrão é 5%, o que significa que cada marcador tem 5% da largura da imagem.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Marcadores maiores e mais visíveis
});
```

Valores menores criam marcadores menos intrusivos que funcionam melhor para imagens detalhadas. Valores maiores tornam os marcadores mais fáceis de ver e clicar em imagens ocupadas ou para usuários em dispositivos móveis.

#### hasDarkBackground

Ative o estilo em modo escuro quando sua página tiver um fundo escuro.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Uma função de callback que é executada sempre que a contagem de comentários muda. Isso é útil para atualizar elementos da interface como badges ou títulos da página.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Opções de configuração herdadas

Como o Image Chat estende o widget de comentários padrão, você pode usar qualquer opção de configuração do widget base do FastComments. Aqui estão algumas opções comumente usadas:

#### locale

Defina o idioma da interface do widget. O FastComments oferece suporte a dezenas de idiomas.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Espanhol
});
```

#### readonly

Deixe todas as conversas em modo somente leitura. Os usuários podem visualizar marcadores e discussões existentes, mas não podem criar novos nem responder.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integre com seu sistema de autenticação usando Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Configuração SSO
    }
});
```

Consulte a documentação de SSO para detalhes completos sobre as opções de autenticação.

#### maxReplyDepth

Controle quantos níveis de profundidade as respostas podem ter. Por padrão, o Image Chat define isso como 0, significando que todos os comentários são planos (sem respostas aninhadas). Você pode alterar isso se quiser conversas encadeadas.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Permite 3 níveis de aninhamento
});
```

### Configuração interna

Essas opções são definidas automaticamente pelo Image Chat e não devem ser sobrescritas:

O `productId` é definido automaticamente como `2` para o Image Chat. A extensão `floating-chat` é carregada automaticamente para fornecer a funcionalidade da janela de chat. O widget detecta automaticamente dispositivos móveis (telas com menos de 768px de largura) e ajusta a interface com janelas de chat em tela cheia.

### Flexibilidade do elemento alvo

O primeiro parâmetro para `FastCommentsImageChat` pode ser tanto um elemento `<img>` diretamente quanto um elemento contêiner com uma imagem dentro:

```javascript
// Elemento de imagem direto
FastCommentsImageChat(document.getElementById('my-image'), config);

// Contêiner com imagem dentro
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

O widget encontrará a imagem automaticamente se você passar um elemento contêiner.

### Exemplo completo

Aqui está um exemplo mostrando várias opções de configuração juntas:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Sua configuração de SSO
    },
    maxReplyDepth: 1
});
```

Para uma lista completa de todas as opções de configuração disponíveis herdadas do widget base, consulte a documentação principal de configuração do FastComments.
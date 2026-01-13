### Visão Geral da API

Collab Chat fornece três endpoints REST da API para gerenciar conversas de texto programaticamente. Esses endpoints permitem que você recupere, crie e exclua anotações sem usar o widget no navegador.

Estes são endpoints públicos que autenticam usuários via cookies do navegador. Eles não usam chaves de API. Os usuários devem estar logados no FastComments no navegador para acessar esses endpoints.

### URL base

Todos os endpoints da API do Collab Chat estão em:

[inline-code-attrs-start title = 'URL base'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Autenticação

Esses endpoints autenticam usuários via cookies do navegador. Eles não usam chaves de API. Os usuários devem estar logados no FastComments no navegador para acessar esses endpoints.

### Obter todas as conversas

Recupere todas as conversas de texto para uma página específica.

#### Endpoint

[inline-code-attrs-start title = 'Endpoint GET'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parâmetros

`tenantId` (parâmetro de caminho, obrigatório) é o seu FastComments Tenant ID.

`urlId` (parâmetro de query, obrigatório) é o identificador da página para a qual você deseja recuperar conversas.

#### Resposta

A resposta inclui o status da API, informações da sessão do usuário atual se autenticado, um array de conversas com seus IDs, URLs e intervalos de texto, um identificador de URL limpo, uma flag indicando se o usuário atual é administrador do site ou moderador, e detalhes de conexão WebSocket para sincronização ao vivo incluindo `tenantIdWS`, `urlIdWS`, e `userIdWS`.

#### Exemplo de requisição

[inline-code-attrs-start title = 'Exemplo de requisição GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Exemplo de resposta

[inline-code-attrs-start title = 'Exemplo de resposta GET'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### Criar conversa

Crie uma nova conversa de texto para uma seleção de texto específica.

#### Endpoint

[inline-code-attrs-start title = 'Endpoint POST'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parâmetros

`tenantId` (parâmetro de caminho, obrigatório) é o seu FastComments Tenant ID.

O corpo da requisição deve ser JSON e incluir os seguintes campos obrigatórios.

`urlId` (string, obrigatório) é o identificador base da página.

`urlIdWithRange` (string, obrigatório) é a URL combinada com o intervalo de texto, por exemplo `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, obrigatório) é o título da página.

`selector` (string, obrigatório) é o caminho DOM para o elemento que contém o texto selecionado.

`range` (string, obrigatório) é o intervalo de texto serializado no formato `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, obrigatório) é o ID do comentário que iniciou este chat.

`broadcastId` (string, obrigatório) é um UUID para sincronização ao vivo para prevenir efeitos de eco.

#### Resposta

A resposta inclui o status da API e o ID da conversa recém-criada.

#### Exemplo de requisição

[inline-code-attrs-start title = 'Exemplo de requisição POST'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### Exemplo de resposta

[inline-code-attrs-start title = 'Exemplo de resposta POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Excluir conversa

Exclua uma conversa de texto existente. Este endpoint requer permissões de administrador ou moderador, ou a conversa deve ter sido criada pelo usuário autenticado.

#### Endpoint

[inline-code-attrs-start title = 'Endpoint DELETE'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parâmetros

`tenantId` (parâmetro de caminho, obrigatório) é o seu FastComments Tenant ID.

`chatId` (parâmetro de caminho, obrigatório) é o ID da conversa a ser excluída.

`broadcastId` (corpo da requisição, obrigatório) é um UUID para sincronização ao vivo.

#### Exemplo de requisição

[inline-code-attrs-start title = 'Exemplo de requisição DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Exemplo de resposta

[inline-code-attrs-start title = 'Exemplo de resposta DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Limitação de taxa

Esses endpoints estão sujeitos à limitação de taxa padrão da API do FastComments. O middleware de limitação de taxa aplica-se por tenant para prevenir abusos ao mesmo tempo em que permite padrões normais de uso.

### Respostas de erro

Todos os endpoints retornam códigos de status HTTP padrão. Uma resposta 400 indica parâmetros de requisição inválidos. Uma resposta 401 significa que a autenticação falhou. Uma resposta 403 indica permissões insuficientes. Uma resposta 404 significa que a conversa não foi encontrada. Uma resposta 429 indica que o limite de taxa foi excedido.

As respostas de erro incluem um corpo JSON com detalhes:

[inline-code-attrs-start title = 'Exemplo de resposta de erro'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Rastreamento de uso

Criar conversas incrementa sua métrica de uso `conversationCreateCount`. Toda a atividade de sincronização via WebSocket incrementa `pubSubMessageCount` e `pubSubBandwidth`. Você pode monitorar essas métricas no painel do FastComments em analytics de uso.

---
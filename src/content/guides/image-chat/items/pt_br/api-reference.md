### API Overview

Image Chat fornece três endpoints REST da API para gerenciar conversas de imagem programaticamente. Esses endpoints permitem recuperar, criar e excluir marcadores sem usar o widget do navegador.

Todos os endpoints da API exigem autenticação e seguem os padrões da API do FastComments. Estes são endpoints públicos que autenticam via cookies do navegador, não por chaves de API.

### Base URL

Todos os endpoints da API do Image Chat estão em:

```
https://fastcomments.com/comment-image-chats
```

### Authentication

Esses endpoints autenticam usuários via cookies do navegador. Eles não usam chaves de API. Os usuários devem estar logados no FastComments em seu navegador para acessar esses endpoints.

### Get All Conversations

Recupere todas as conversas de imagem para uma imagem específica.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (path parameter, required) é o seu FastComments Tenant ID.

`urlId` (query parameter, required) é o identificador da imagem para a qual você deseja recuperar conversas.

#### Response

A resposta inclui o status da API, informações da sessão do usuário atual se autenticado, um array de conversas com seus IDs, URLs e coordenadas X/Y, um identificador de URL limpo, uma flag indicando se o usuário atual é administrador do site ou moderador, e detalhes de conexão WebSocket para sincronização ao vivo incluindo `tenantIdWS`, `urlIdWS`, e `userIdWS`.

#### Example Request

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Example Response

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### Create Conversation

Crie uma nova conversa de imagem para uma localização específica em uma imagem.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (path parameter, required) é o seu FastComments Tenant ID.

O corpo da requisição deve ser JSON e incluir estes campos obrigatórios.

`urlId` (string, required) é o identificador base da página.

`windowUrlId` (string, required) é a URL combinada com a fonte da imagem e as coordenadas, por exemplo `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, required) é o título da página.

`src` (string, required) é a URL da fonte da imagem.

`x` (number, required) é a coordenada X em porcentagem (0-100).

`y` (number, required) é a coordenada Y em porcentagem (0-100).

`createdFromCommentId` (string, required) é o ID do comentário que iniciou este chat.

`broadcastId` (string, required) é um UUID para sincronização ao vivo para prevenir efeitos de eco.

#### Response

A resposta inclui o status da API e o ID da conversa recém-criada.

#### Example Request

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### Example Response

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Delete Conversation

Exclua uma conversa de imagem existente. Este endpoint exige permissões de administrador ou moderador, ou a conversa deve ter sido criada pelo usuário autenticado.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (path parameter, required) é o seu FastComments Tenant ID.

`chatId` (path parameter, required) é o ID da conversa a ser excluída.

`broadcastId` (request body, required) é um UUID para sincronização ao vivo.

#### Example Request

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Example Response

```json
{
  "status": "success"
}
```

### Coordinate System

As coordenadas X e Y são armazenadas como porcentagens das dimensões da imagem. X representa a posição horizontal a partir da borda esquerda (0 = borda esquerda, 100 = borda direita). Y representa a posição vertical a partir da borda superior (0 = topo, 100 = parte inferior).

Esses valores percentuais podem incluir casas decimais para maior precisão. Por exemplo, x: 25.5 significa 25.5% a partir da borda esquerda da imagem.

### Rate Limiting

Esses endpoints estão sujeitos à limitação de taxa padrão da API do FastComments. O middleware de limite de taxa aplica-se por tenant para prevenir abuso enquanto permite padrões normais de uso.

### Error Responses

Todos os endpoints retornam códigos de status HTTP padrão. Uma resposta 400 indica parâmetros de requisição inválidos. Uma resposta 401 significa que a autenticação falhou. Uma resposta 403 indica permissões insuficientes. Uma resposta 404 significa que a conversa não foi encontrada. Uma resposta 429 indica que o limite de taxa foi excedido.

As respostas de erro incluem um corpo JSON com detalhes:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Usage Tracking

Criar conversas incrementa sua métrica de uso `conversationCreateCount`. Toda atividade de sincronização via WebSocket incrementa `pubSubMessageCount` e `pubSubBandwidth`. Você pode monitorar essas métricas no painel do FastComments em usage analytics.

---
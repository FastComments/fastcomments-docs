### Visão Geral da API

O Image Chat fornece três endpoints de API REST para gerenciar conversas de imagens programaticamente. Esses endpoints permitem recuperar, criar e excluir marcadores sem usar o widget do navegador.

Todos os endpoints da API exigem autenticação e seguem os padrões padrão da API do FastComments. Estes são endpoints públicos que autenticam via cookies do navegador, não via chaves de API.

### URL Base

Todos os endpoints da API do Image Chat estão sob:

```
https://fastcomments.com/comment-image-chats
```

### Autenticação

Esses endpoints autenticam os usuários via cookies do navegador. Eles não usam chaves de API. Os usuários devem estar logados no FastComments em seu navegador para acessar esses endpoints.

### Obter Todas as Conversas

Recupera todas as conversas de imagem para uma imagem específica.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parâmetros

`tenantId` (parâmetro de caminho, obrigatório) é o seu Tenant ID do FastComments.

`urlId` (parâmetro de consulta, obrigatório) é o identificador da imagem para a qual você deseja recuperar as conversas.

#### Resposta

A resposta inclui o status da API, informações da sessão do usuário atual se autenticado, um array de conversas com seus IDs, URLs e coordenadas X/Y, um identificador de URL limpo, uma flag indicando se o usuário atual é um administrador ou moderador do site, e detalhes da conexão WebSocket para sincronização ao vivo, incluindo `tenantIdWS`, `urlIdWS` e `userIdWS`.

#### Exemplo de Requisição

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Exemplo de Resposta

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

### Criar Conversa

Cria uma nova conversa de imagem para um local específico em uma imagem.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parâmetros

`tenantId` (parâmetro de caminho, obrigatório) é o seu Tenant ID do FastComments.

O corpo da requisição deve ser JSON e incluir estes campos obrigatórios.

`urlId` (string, obrigatório) é o identificador base da página.

`windowUrlId` (string, obrigatório) é a URL combinada com a origem da imagem e as coordenadas, por exemplo `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, obrigatório) é o título da página.

`src` (string, obrigatório) é a URL de origem da imagem.

`x` (número, obrigatório) é a coordenada X como porcentagem (0-100).

`y` (número, obrigatório) é a coordenada Y como porcentagem (0-100).

`createdFromCommentId` (string, obrigatório) é o ID do comentário que iniciou este chat.

`broadcastId` (string, obrigatório) é um UUID para sincronização ao vivo, para evitar efeitos de eco.

#### Resposta

A resposta inclui o status da API e o ID da conversa recém-criada.

#### Exemplo de Requisição

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

#### Exemplo de Resposta

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Excluir Conversa

Exclui uma conversa de imagem existente. Este endpoint requer permissões de administrador ou moderador, ou a conversa deve ter sido criada pelo usuário autenticado.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parâmetros

`tenantId` (parâmetro de caminho, obrigatório) é o seu Tenant ID do FastComments.

`chatId` (parâmetro de caminho, obrigatório) é o ID da conversa a ser excluída.

`broadcastId` (corpo da requisição, obrigatório) é um UUID para sincronização ao vivo.

#### Exemplo de Requisição

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Exemplo de Resposta

```json
{
  "status": "success"
}
```

### Sistema de Coordenadas

As coordenadas X e Y são armazenadas como porcentagens das dimensões da imagem. X representa a posição horizontal a partir da borda esquerda (0 = borda esquerda, 100 = borda direita). Y representa a posição vertical a partir da borda superior (0 = borda superior, 100 = borda inferior).

Esses valores percentuais podem incluir decimais para maior precisão. Por exemplo, x: 25.5 significa 25,5% a partir da borda esquerda da imagem.

### Limitação de Taxa

Esses endpoints estão sujeitos à limitação de taxa padrão da API do FastComments. O middleware de limite de taxa é aplicado por tenant para evitar abusos, permitindo padrões de uso normais.

### Respostas de Erro

Todos os endpoints retornam códigos de status HTTP padrão. Uma resposta 400 indica parâmetros de requisição inválidos. Uma resposta 401 significa que a autenticação falhou. Uma resposta 403 indica permissões insuficientes. Uma resposta 404 significa que a conversa não foi encontrada. Uma resposta 429 indica que o limite de taxa foi excedido.

As respostas de erro incluem um corpo JSON com detalhes:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Rastreamento de Uso

A criação de conversas incrementa sua métrica de uso `conversationCreateCount`. Toda atividade de sincronização WebSocket incrementa `pubSubMessageCount` e `pubSubBandwidth`. Você pode monitorar essas métricas no seu painel do FastComments em análises de uso.
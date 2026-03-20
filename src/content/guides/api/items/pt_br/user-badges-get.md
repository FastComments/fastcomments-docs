---
Este endpoint permite recuperar as insígnias de usuários com base em vários critérios.

Exemplo de requisição:

[inline-code-attrs-start title = 'Listar Insígnias de Usuário - Exemplo GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Você pode adicionar vários parâmetros de consulta para filtrar os resultados:

- `userId` - Obter insígnias para um usuário específico
- `badgeId` - Obter instâncias de uma insígnia específica
- `type` - Filtrar por tipo de insígnia (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. Veja a estrutura UserBadge para a lista completa)
- `displayedOnComments` - Filtrar por se a insígnia é exibida em comentários (true/false)
- `limit` - Número máximo de insígnias a retornar (padrão 30, máximo 200)
- `skip` - Número de insígnias a ignorar (para paginação)

Exemplo de resposta:

[inline-code-attrs-start title = 'Resposta'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadges": [
    {
      "id": "badge123",
      "userId": "user456",
      "badgeId": "badgeDef789",
      "fromTenantId": "tenant001",
      "createdAt": 1650532511000,
      "receivedAt": 1650532511000,
      "type": 14,
      "name": "Special Contributor",
      "description": "Awarded to special contributors to our community",
      "displayLabel": "Special",
      "backgroundColor": "#4a5568",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 1
    },
    {
      "id": "badge124",
      "userId": "user456",
      "badgeId": "badgeDef790",
      "fromTenantId": "tenant001",
      "createdAt": 1650532598000,
      "receivedAt": 1650532598000,
      "type": 0,
      "threshold": 100,
      "name": "Centurion",
      "description": "Made 100 comments",
      "displayLabel": "100",
      "backgroundColor": "#2b6cb0",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 2
    }
  ]
}
[inline-code-end]

Possíveis respostas de erro:

[inline-code-attrs-start title = 'Erro: ID do Tenant ausente'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erro: Limite inválido'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]
---
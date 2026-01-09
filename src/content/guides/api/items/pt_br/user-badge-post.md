Este endpoint permite criar uma nova atribuição de distintivo (badge) para um usuário.

Exemplo de requisição:

[inline-code-attrs-start title = 'Exemplo de requisição POST'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "userId": "user456",
  "badgeId": "badgeDef789",
  "displayedOnComments": true
}'
[inline-code-end]

O corpo da requisição deve conter os seguintes parâmetros:

- `userId` (required) - The ID of the user to assign the badge to
- `badgeId` (required) - The ID of the badge to assign
- `displayedOnComments` (optional) - Whether the badge should be displayed on the user's comments (defaults to true)

Notas importantes:
1. O distintivo (badge) deve existir e estar habilitado no catálogo de badges do seu tenant
2. Você só pode atribuir badges a usuários que pertencem ao seu tenant ou que tenham comentado em seu site

Exemplo de resposta:

[inline-code-attrs-start title = 'Resposta'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadge": {
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
  }
}
[inline-code-end]

Possíveis respostas de erro:

[inline-code-attrs-start title = 'Erro: Tenant ID ausente'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erro: ID do usuário ausente'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "User ID (body param: userId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erro: ID do badge ausente'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-badge-id",
  "reason": "Badge ID (body param: badgeId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erro: Badge não encontrado'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "badge-not-found",
  "reason": "The badge badgeDef789 was not found or is not enabled."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erro: Usuário não autorizado'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "unauthorized-user",
  "reason": "You can only add badges to users who belong to your tenant or have commented on your site."
}
[inline-code-end]
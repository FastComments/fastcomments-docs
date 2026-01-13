Este endpoint le permite obtener insignias de usuario basadas en varios criterios.

Ejemplo de Solicitud:

[inline-code-attrs-start title = 'Ejemplo de Solicitud GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Puede agregar varios parámetros de consulta para filtrar los resultados:

- `userId` - Obtener insignias para un usuario específico
- `badgeId` - Obtener instancias de una insignia específica
- `type` - Filtrar por tipo de insignia (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. Ver estructura de UserBadge para la lista completa)
- `displayedOnComments` - Filtrar por si la insignia se muestra en comentarios (true/false)
- `limit` - Número máximo de insignias a devolver (predeterminado 30, máximo 200)
- `skip` - Número de insignias a omitir (para paginación)

Ejemplo de Respuesta:

[inline-code-attrs-start title = 'Respuesta'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Posibles Respuestas de Error:

[inline-code-attrs-start title = 'Error: Falta ID de Inquilino'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Error: Límite Inválido'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]

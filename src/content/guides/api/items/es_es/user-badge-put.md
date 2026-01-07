Este endpoint le permite actualizar una asignación de insignia de usuario.

Actualmente, la única propiedad que puede actualizarse es `displayedOnComments`, que controla si la insignia se muestra en los comentarios del usuario.

Ejemplo de Solicitud:

[inline-code-attrs-start title = 'Ejemplo de Solicitud PUT'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X PUT "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "displayedOnComments": false
}'
[inline-code-end]

Ejemplo de Respuesta:

[inline-code-attrs-start title = 'Respuesta'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
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

[inline-code-attrs-start title = 'Error: Falta ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-id",
  "reason": "The User Badge ID (url param: id) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Error: No Encontrado'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge badge123 was not found."
}
[inline-code-end]

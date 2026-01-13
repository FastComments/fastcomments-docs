[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de actualizar un `Moderator` por `id`.

Actualizar un `Moderator` tiene las siguientes restricciones:

- Los siguientes valores no pueden proporcionarse al actualizar un `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Cuando se especifica un `userId`, ese usuario debe existir.
- Cuando se especifica un `userId`, deben pertenecer al mismo `tenantId` especificado en los parámetros de consulta.
- Dos moderadores en el mismo inquilino no pueden agregarse con el mismo `email`.
- No puede cambiar el `tenantId` asociado con un `Moderator`.

[inline-code-attrs-start title = 'Ejemplo cURL de PATCH de Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de PATCH de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de PATCH de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

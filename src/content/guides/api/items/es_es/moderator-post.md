[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Esta ruta proporciona la capacidad de agregar un único `Moderator`.

Crear un `Moderator` tiene las siguientes restricciones:

- Siempre debe proporcionarse un `name` y `email`. Un `userId` es opcional.
- Los siguientes valores no pueden proporcionarse al crear un `Moderator`:
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

Podemos crear un `Moderator` para un usuario del cual solo conocemos el email:

[inline-code-attrs-start title = 'Ejemplo cURL de Creación de Moderator vía Email'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

O podemos crear un `Moderator` para un usuario que pertenece a nuestro inquilino, para rastrear sus estadísticas de moderación:

[inline-code-attrs-start title = 'Ejemplo cURL de Creación de Moderator vía Usuario de Inquilino'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Creación de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Creación de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    moderator?: Moderator; // We return the complete created moderator on success.
}
[inline-code-end]

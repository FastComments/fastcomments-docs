[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Esta ruta proporciona la capacidad de invitar a un único `Moderator`.

Las siguientes restricciones existen para enviar un email de invitación a un `Moderator`:
- El `Moderator` ya debe existir.
- El `fromName` no puede tener más de `100 caracteres`.

**Notas:**
- Si un usuario con el email proporcionado ya existe, será invitado a moderar los comentarios de su inquilino.
- Si un usuario con el email proporcionado **no existe** el enlace de invitación los guiará a través de la creación de su cuenta.
- La invitación expirará después de `30 días`.

Podemos crear un `Moderator` para un usuario del cual solo conocemos el email:

[inline-code-attrs-start title = 'Ejemplo cURL de Invitación de Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Esto enviará un email como `Bob de TenantName te está invitando a ser moderador...`

[inline-code-attrs-start title = 'Estructura de Solicitud de Invitación de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** The email sent to the user will appear to be sent from this name. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Invitación de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

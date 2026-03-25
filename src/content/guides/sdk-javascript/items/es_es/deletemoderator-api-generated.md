## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| sendEmail | string | No |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2d4a6c';
const moderatorId: string = 'moderator_84a1b9c2';
const sendEmail: string = 'true';
const result: FlagCommentPublic200Response = await deleteModerator(tenantId, moderatorId, sendEmail);
[inline-code-end]

---
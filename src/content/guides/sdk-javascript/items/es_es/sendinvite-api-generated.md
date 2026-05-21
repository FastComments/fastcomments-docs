## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| fromName | string | Sí |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de sendInvite'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-128';
const id: string = 'comment-8421f';
const fromName: string = 'Marcus Lindström';
const note: string | undefined = undefined; // ejemplo de parámetro opcional
const response: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName);
[inline-code-end]
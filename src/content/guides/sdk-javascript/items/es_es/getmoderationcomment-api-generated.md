## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| commentId | string | Sí |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_62b8f9a3e1d4';
const includeEmail: boolean = true;
const includeIP: boolean = false;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4In0.signature';
const response: ModerationAPICommentResponse = await getModerationComment(commentId, includeEmail, includeIP, sso);
[inline-code-end]

---
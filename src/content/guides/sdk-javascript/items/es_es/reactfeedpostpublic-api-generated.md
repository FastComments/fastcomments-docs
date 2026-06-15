---
## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| postId | string | Sí |  |
| reactBodyParams | ReactBodyParams | Sí |  |
| isUndo | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de reactFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const postId: string = 'feedpost_78901';
const reactBodyParams: ReactBodyParams = { reaction: 'like', emoji: '👍' };
const isUndo: boolean = false;
const broadcastId: string = 'broadcast_2026_06_15_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signature';

const response: ReactFeedPostPublic200Response = await reactFeedPostPublic(
  tenantId,
  postId,
  reactBodyParams,
  isUndo,
  broadcastId,
  sso
);
[inline-code-end]

---
---
## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| postId | string | Sim |  |
| reactBodyParams | ReactBodyParams | Sim |  |
| isUndo | boolean | Não |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de reactFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
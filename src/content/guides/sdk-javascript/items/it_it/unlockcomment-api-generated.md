## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| broadcastId | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`LockComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockComment200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di unLockComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9d4f2b';
const commentId: string = 'cmt_8a3e1f';
const broadcastId: string = 'broadcast_2026_03_25';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';

const result: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---
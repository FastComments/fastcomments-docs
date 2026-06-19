## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| commentIds | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckBlockedCommentsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di checkedCommentsForBlocked'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const commentIds: string = 'cmt_1001,cmt_1002';
const resultWithoutSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds);

const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dummy.payload';
const resultWithSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds, sso);
[inline-code-end]

---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentIds | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckBlockedCommentsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'checkedCommentsForBlocked Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const commentIds: string = 'cmt_1001,cmt_1002';
const resultWithoutSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds);

const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dummy.payload';
const resultWithSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds, sso);
[inline-code-end]
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |

## Yanıt

Döndürür: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## Örnek

[inline-code-attrs-start title = 'flagComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme-corp_01';
const commentId: string = 'cmt_5f8d7a2b3c4e';
const anonUserId: string = 'anon_9c3a1f0e';
const response: FlagCommentResponse = await flagComment(tenantId, commentId, undefined, anonUserId);
[inline-code-end]

---
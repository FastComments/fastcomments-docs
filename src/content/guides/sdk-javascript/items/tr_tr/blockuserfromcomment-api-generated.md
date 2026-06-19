## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| blockFromCommentParams | BlockFromCommentParams | Evet |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |

## Yanıt

Dönen değer: [`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## Örnek

[inline-code-attrs-start title = 'blockUserFromComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = '5f9a3b2c-1d3e-4b6f-8a9c-12d345ef6789';
const blockFromCommentParams: BlockFromCommentParams = { reason: 'Repeated spam', durationDays: 30, notifyModerator: true };
const userId: string = 'user-1024';
const result: BlockSuccess = await blockUserFromComment(tenantId, id, blockFromCommentParams, userId);
[inline-code-end]

---
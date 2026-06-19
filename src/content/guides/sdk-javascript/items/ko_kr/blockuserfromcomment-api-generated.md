## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| blockFromCommentParams | BlockFromCommentParams | 예 |  |
| userId | string | 아니요 |  |
| anonUserId | string | 아니요 |  |

## 응답

반환: [`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## 예제

[inline-code-attrs-start title = 'blockUserFromComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = '5f9a3b2c-1d3e-4b6f-8a9c-12d345ef6789';
const blockFromCommentParams: BlockFromCommentParams = { reason: 'Repeated spam', durationDays: 30, notifyModerator: true };
const userId: string = 'user-1024';
const result: BlockSuccess = await blockUserFromComment(tenantId, id, blockFromCommentParams, userId);
[inline-code-end]

---
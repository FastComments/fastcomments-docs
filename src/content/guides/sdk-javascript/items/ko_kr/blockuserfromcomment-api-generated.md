---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| blockFromCommentParams | BlockFromCommentParams | 예 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |

## 응답

반환: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'blockUserFromComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = 'comment_7f3b2a9c';
const blockFromCommentParams: BlockFromCommentParams = {
  reason: 'Repeated abusive language and targeted harassment',
  durationDays: 90,
  preventReposting: true
};
const userId: string = 'user_12345';
const anonUserId: string = 'anon_98765';

const result: BlockFromCommentPublic200Response = await blockUserFromComment(
  tenantId,
  id,
  blockFromCommentParams,
  userId,
  anonUserId
);
[inline-code-end]

---
## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 예 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |

## 응답

반환: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'unBlockUserFromComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f3e1';
const id: string = 'comment_9b2a4f';
const unBlockFromCommentParams: UnBlockFromCommentParams = {
  reason: 'Reviewed by moderation team — reinstated',
  moderatorId: 'mod_21',
  unblockedAt: new Date().toISOString()
};
const userId: string = 'user_42';
const anonUserId: string = 'anon_e7f9';
const result: UnBlockCommentPublic200Response = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId, anonUserId);
[inline-code-end]

---
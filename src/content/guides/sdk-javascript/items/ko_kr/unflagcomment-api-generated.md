## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## 응답

반환: [`UnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnFlagCommentResponse.ts)

## 예제

[inline-code-attrs-start title = 'unFlagComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_123456";
const userId: string = "usr_98765";

const result: UnFlagCommentResponse = await unFlagComment(tenantId, commentId, userId);

const anonCommentId: string = "cmt_123457";
const anonUserId: string = "anon_abc123";

const anonResult: UnFlagCommentResponse = await unFlagComment(tenantId, anonCommentId, undefined, anonUserId);
[inline-code-end]
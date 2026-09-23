## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| reviewed | boolean | 아니오 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예시

[inline-code-attrs-start title = 'postSetCommentReviewStatus 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const reviewed: boolean = true;
const broadcastId: string = "brd_001";
const sso: string = "sso_token_abc";

const result: APIEmptyResponse = await postSetCommentReviewStatus(
  tenantId,
  commentId,
  reviewed,
  broadcastId,
  sso
);
[inline-code-end]
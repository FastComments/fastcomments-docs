## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| includeEmail | boolean | 아니오 |  |
| includeIP | boolean | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## 예시

[inline-code-attrs-start title = 'getModerationComment 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // 필수 매개변수만 사용하여 호출
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // 선택 매개변수와 함께 호출
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const sso: string = "sso-token-abc123";
  const detailedResponse: ModerationAPICommentResponse = await getModerationComment(
    tenantId,
    commentId,
    includeEmail,
    includeIP,
    sso
  );
}
[inline-code-end]
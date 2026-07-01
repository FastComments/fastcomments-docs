## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| commentId | string | Yes |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 응답

반환: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## 예시

[inline-code-attrs-start title = 'getModerationComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // 전체 매개변수 집합
  const commentId: string = "cmt_12345abc";
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const tenantId: string = "tenant_9876";
  const sso: string = "sso_token_xyz";

  const fullResult: GetModerationCommentResponse = await getModerationComment(
    commentId,
    includeEmail,
    includeIP,
    tenantId,
    sso
  );

  // 필수 인수만 사용한 최소 호출
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // 결과를 필요에 따라 사용...
}
[inline-code-end]
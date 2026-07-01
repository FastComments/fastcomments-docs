## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| commentId | string | 예 |  |
| tenantId | string | 아니오 |  |
| sso | string | 아니오 |  |

## Response

반환: [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Example

[inline-code-attrs-start title = 'getModerationCommentText 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // 필수 매개변수만 사용하여 호출
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // 선택적 매개변수와 함께 호출
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]
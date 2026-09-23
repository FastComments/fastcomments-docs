## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| includeEmail | boolean | いいえ |  |
| includeIP | boolean | いいえ |  |
| sso | string | いいえ |  |

## Response

返却: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## Example

[inline-code-attrs-start title = 'getModerationComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // 必要なパラメータのみで呼び出す
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // オプションのパラメータで呼び出す
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

---
## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| includeEmail | boolean | Ні |  |
| includeIP | boolean | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Виклик лише з обов'язковими параметрами
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // Виклик з необов'язковими параметрами
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
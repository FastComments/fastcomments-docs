## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentChildren'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchChildren() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const ssoToken: string = "sso_user_abc123";

  const responseWithSSO: ModerationAPIChildCommentsResponse = await getCommentChildren(tenantId, commentId, ssoToken);
  const responseWithoutSSO: ModerationAPIChildCommentsResponse = await getCommentChildren(tenantId, commentId);
}

fetchChildren();
[inline-code-end]
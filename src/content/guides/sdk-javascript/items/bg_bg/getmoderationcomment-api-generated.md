## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| includeEmail | boolean | Не |  |
| includeIP | boolean | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Извикване само с задължителните параметри
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // Извикване с незадължителните параметри
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
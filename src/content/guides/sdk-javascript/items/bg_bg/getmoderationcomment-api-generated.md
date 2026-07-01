## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Отговор

Връща: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'getModerationComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Пълен набор от параметри
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

  // Минимално извикване, използвайки само задължителния аргумент
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Използвайте резултатите според нуждите...
}
[inline-code-end]
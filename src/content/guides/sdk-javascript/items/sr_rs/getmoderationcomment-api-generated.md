## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| commentId | string | Да |  |
| includeEmail | boolean | Не |  |
| includeIP | boolean | Не |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'Primer getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Пун скуп параметара
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

  // Минимални позив користећи само потребан аргумент
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Користите резултате по потреби...
}
[inline-code-end]
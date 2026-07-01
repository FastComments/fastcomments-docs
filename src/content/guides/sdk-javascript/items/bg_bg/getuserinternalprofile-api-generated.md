## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| commentId | string | Не |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`GetUserInternalProfileResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse1.ts)

## Пример

[inline-code-attrs-start title = 'getUserInternalProfile Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const fullProfile: GetUserInternalProfileResponse1 = await getUserInternalProfile({
    commentId: "cmt_12345",
    tenantId: "tenant_67890",
    sso: "sso_token_abcdef"
  });

  const partialProfile: GetUserInternalProfileResponse1 = await getUserInternalProfile({
    commentId: "cmt_98765"
  });
})();
[inline-code-end]
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| sendEmail | string | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'deleteModerator Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const moderatorId: string = "mod_9876";
  const notifyEmail: string = "admin@company.com";

  const resultWithEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId, notifyEmail);
  const resultWithoutEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId);
})();
[inline-code-end]
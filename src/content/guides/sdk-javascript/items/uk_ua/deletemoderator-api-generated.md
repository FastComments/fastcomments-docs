## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| sendEmail | string | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад використання deleteModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8f3b2a7c';
  const moderatorId: string = 'mod_4c12f9b2';
  const responseWithoutEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId);
  const responseWithEmail: APIEmptyResponse = await deleteModerator(tenantId, moderatorId, 'true');
  console.log(responseWithoutEmail, responseWithEmail);
})();
[inline-code-end]

---
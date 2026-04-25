## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createModeratorBody | CreateModeratorBody | Так |  |

## Відповідь

Повертає: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModerator200Response.ts)

## Приклад

[inline-code-attrs-start title = 'createModerator Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3b6c";
const optionalConfig: CustomConfigParameters = { moderationThreshold: 5, escalateOnRepeatedOffenses: true };
const newModerator: CreateModeratorBody = {
  email: "lina.gomez@dailynews.com",
  fullName: "Lina Gomez",
  role: "senior_moderator",
  enabled: true,
  notifyByEmail: true,
  customConfig: optionalConfig
};
const response: CreateModerator200Response = await createModerator(tenantId, newModerator);
[inline-code-end]

---
## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createModeratorBody | CreateModeratorBody | Да |  |

## Одговор

Враћа: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Пример

[inline-code-attrs-start title = 'createModerator Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const moderatorPayload: CreateModeratorBody = {
  userId: "user_9876",
  // опционално поље; може се изоставити ако није потребно
  notes: "Temporary moderator for event"
};

const response: CreateModeratorResponse = await createModerator(tenantId, moderatorPayload);
[inline-code-end]
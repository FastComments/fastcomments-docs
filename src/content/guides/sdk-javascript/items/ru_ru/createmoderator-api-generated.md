## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createModeratorBody | CreateModeratorBody | Да |  |

## Ответ

Возвращает: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const moderatorPayload: CreateModeratorBody = {
  userId: "user_9876",
  // необязательное поле; может быть опущено, если не требуется
  notes: "Temporary moderator for event"
};

const response: CreateModeratorResponse = await createModerator(tenantId, moderatorPayload);
[inline-code-end]

---
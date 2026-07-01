---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createModeratorBody | CreateModeratorBody | Yes |  |

## Отговор

Връща: [`CreateModeratorResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const moderatorPayload: CreateModeratorBody = {
    name: "Alice Johnson",
    email: "alice.johnson@example.com"
    // опционални полета като описание са пропуснати
  };
  const result: CreateModeratorResponse1 = await createModerator(tenantId, moderatorPayload);
  console.log(result);
}

runExample();
[inline-code-end]

---
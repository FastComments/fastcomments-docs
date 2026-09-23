## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| fromName | string | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'sendInvite Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-987654321";
const id: string = "user-123456789";
const fromName: string = "Alice Johnson";

const result: APIEmptyResponse = await sendInvite(tenantId, id, fromName);
console.log(result);
[inline-code-end]

---
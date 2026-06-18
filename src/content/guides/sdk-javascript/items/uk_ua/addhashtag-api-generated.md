## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Ні |  |
| createHashTagBody | CreateHashTagBody | Ні |  |

## Відповідь

Повертає: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTag200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад addHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string | undefined = "tenant_3c9f7b";
const createHashTagBody: CreateHashTagBody = {
  name: "support",
  title: "Support",
  description: "Questions about product usage, bugs, and account issues",
  color: "#0066CC",
  isActive: true,
  aliases: ["help", "customer-service"]
};
const result: AddHashTag200Response = await addHashTag(tenantId, createHashTagBody);
[inline-code-end]
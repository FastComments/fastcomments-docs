Creates a new trial account for an AI agent without a human signup. No API key is needed to call this.

Створює новий пробний обліковий запис для AI‑агента без реєстрації людини. Для виклику цього не потрібен API‑ключ.

The response contains the tenant id, an API key that works immediately against the REST API and the
MCP server, and a claim URL. Give the claim URL to the human you are working for: opening it while
logged in to FastComments attaches the account to them. Unclaimed accounts, and their keys, are
deleted 72 hours after creation. Until claimed, the account has the standard trial limits.

Відповідь містить ідентифікатор орендаря, API‑ключ, який одразу працює з REST API та сервером MCP, а також URL‑адресу для підтвердження. Передайте URL‑адресу підтвердження людині, для якої ви працюєте: відкриття її під час входу в FastComments прив’язує обліковий запис до неї. Непідтверджені облікові записи та їхні ключі видаляються через 72 години після створення. Поки обліковий запис не підтверджено, він має стандартні пробні обмеження.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Так |  |

## Відповідь

Повертає: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
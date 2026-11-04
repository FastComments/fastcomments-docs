Створює новий пробний обліковий запис для AI-агента без участі людини в реєстрації. Для виклику не потрібен API-ключ.

Відповідь містить ідентифікатор тенанта, API-ключ, який одразу працює з REST API та
MCP-сервером, а також URL для привласнення облікового запису. Передайте цей URL людині, на яку ви працюєте: відкриття його,
коли ви увійшли в FastComments, прив'язує обліковий запис до неї. Непривласнені облікові записи та їхні ключі
видаляються через 72 години після створення. До привласнення обліковий запис має стандартні обмеження пробного періоду.

## Параметри

| Назва | Тип | Обов'язковий | Опис |
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
  contactEmail: "admin@acme.example.com", // необов'язково
  planId: 3 // необов'язково
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
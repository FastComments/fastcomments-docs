Создает новую пробную учетную запись для AI‑агента без регистрации человеком. Для вызова не требуется API‑ключ.

Ответ содержит идентификатор арендатора, API‑ключ, который сразу же работает с REST API и сервером MCP, а также URL‑адрес для привязки. Передайте URL‑адрес привязки человеку, для которого вы работаете: открытие его, будучи авторизованным в FastComments, привязывает к нему учетную запись. Непривязанные учетные записи и их ключи удаляются через 72 часа после создания. Пока учетная запись не привязана, она имеет стандартные пробные ограничения.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Yes |  |

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
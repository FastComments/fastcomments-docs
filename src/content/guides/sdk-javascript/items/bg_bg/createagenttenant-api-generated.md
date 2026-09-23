Създава нов пробен акаунт за AI агент без човешка регистрация. Не е необходим API ключ за извикване.

Отговорът съдържа идентификатора на наемателя, API ключ, който работи незабавно срещу REST API и MCP сървъра, и URL за заявяване. Дайте URL за заявяване на човека, за когото работите: отварянето му, докато сте влезли в FastComments, свързва акаунта с него. Незаписаните акаунти и техните ключове се изтриват 72 часа след създаването. Докато не бъде заявен, акаунтът има стандартните пробни ограничения.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Yes |  |

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Example

[inline-code-attrs-start title = 'createAgentTenant Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // по избор
  planId: 3 // по избор
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
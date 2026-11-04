Креира нови пробни налог за AI агента без људске регистрације. За позив овог API-ја није потребан API кључ.

Одговор садржи ID станара (tenant id), API кључ који одмах ради са REST API-јем и MCP сервером, као и URL за преузимање налога (claim URL). Дајте тај URL људском кориснику за ког радите: отварање URL-а док је пријављен на FastComments везује налог за њега. Непреузети налози, заједно са својим кључевима, бришу се 72 сата након креирања. Док налог није преузет, на њега се односе стандардна ограничења пробног периода.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Да |  |

## Одговор

Враћа: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Пример

[inline-code-attrs-start title = 'createAgentTenant пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // опционално
  planId: 3 // опционално
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
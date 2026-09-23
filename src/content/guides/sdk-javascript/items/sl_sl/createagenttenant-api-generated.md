Ustvari nov preizkusni račun za AI agenta brez človeške registracije. Za klic tega ni potreben API ključ.

Odgovor vsebuje ID najemnika, API ključ, ki takoj deluje proti REST API-ju in strežniku MCP, ter URL za prevzem. URL za prevzem dajte osebi, za katero delate: odpiranje tega URL-ja med prijavo v FastComments poveže račun z njo. Neprevzeti računi in njihovi ključi se izbrišejo 72 ur po ustvarjanju. Dokler ni prevzet, ima račun standardne preizkusne omejitve.

## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| createAgentTenantBody | CreateAgentTenantBody | Da |  |

## Response

Vrne: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Example

[inline-code-attrs-start title = 'createAgentTenant Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // neobvezno
  planId: 3 // neobvezno
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
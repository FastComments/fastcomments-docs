---
Stvara novi probni račun za AI agenta bez ljudske registracije. Nije potreban API ključ za pozivanje ove funkcije.

Odgovor sadrži ID najmodavca, API ključ koji odmah funkcionira protiv REST API-ja i MCP poslužitelja, te URL za preuzimanje računa. Dajte URL za preuzimanje računa osobi za koju radite: otvaranjem tog URL-a dok ste prijavljeni u FastComments, račun se povezuje s tom osobom. Nepreuzeti računi i njihovi ključevi brišu se 72 sata nakon stvaranja. Dok se račun ne preuzme, ima standardna probna ograničenja.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| createAgentTenantBody | CreateAgentTenantBody | Da |  |

## Odgovor

Vraća: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // opcional
  planId: 3 // opcional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]

---
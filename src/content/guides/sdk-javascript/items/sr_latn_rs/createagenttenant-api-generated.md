Kreira novi probni nalog za AI agenta bez ljudske registracije. Nije potreban API ključ za pozivanje ove funkcije.

Odgovor sadrži ID zakupca, API ključ koji odmah funkcioniše protiv REST API‑ja i MCP servera, i URL za preuzimanje prava. Dajte URL za preuzimanje prava osobi za koju radite: otvaranje tog URL‑a dok ste prijavljeni u FastComments povezuje nalog sa tom osobom. Nepreuzeti nalozi i njihovi ključevi brišu se 72 sata nakon kreiranja. Dok se ne preuzmu, nalog ima standardna probna ograničenja.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| createAgentTenantBody | CreateAgentTenantBody | Da |  |

## Odgovor

Vraća: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // opcionalno
  planId: 3 // opcionalno
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
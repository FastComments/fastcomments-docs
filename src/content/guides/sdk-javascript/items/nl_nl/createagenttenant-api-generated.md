Maakt een nieuw proefaccount aan voor een AI-agent zonder menselijke registratie. Er is geen API-sleutel nodig om dit aan te roepen.

Het antwoord bevat de tenant-id, een API-sleutel die direct werkt tegen de REST API en de
MCP-server, en een claim-URL. Geef de claim-URL aan de mens voor wie u werkt: door deze te openen
terwijl u bent ingelogd bij FastComments wordt het account aan die persoon gekoppeld. Niet-geclaimde accounts, en hun sleutels, worden
72 uur na aanmaak verwijderd. Tot het geclaimd is, gelden voor het account de standaard proeflimieten.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Ja |  |

## Antwoord

Geeft terug: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createAgentTenant Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optioneel
  planId: 3 // optioneel
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
Opretter en ny prøvekonto til en AI-agent uden menneskelig tilmelding. Der kræves ingen API-nøgle for at kalde dette.

Svaret indeholder tenant-id'et, en API-nøgle, der fungerer med det samme mod REST API'et og MCP-serveren, samt en claim-URL. Giv claim-URL'en til det menneske, du arbejder for: når den åbnes, mens man er logget ind på FastComments, tilknyttes kontoen til denne person. Ikke-claimede konti, og deres nøgler, slettes 72 timer efter oprettelsen. Indtil den er claimet, har kontoen de standard begrænsninger for prøveversioner.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Ja |  |

## Svar

Returnerer: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'createAgentTenant Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
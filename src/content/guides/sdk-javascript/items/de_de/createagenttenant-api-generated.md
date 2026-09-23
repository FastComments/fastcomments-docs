Erstellt ein neues Testkonto für einen KI‑Agenten ohne menschliche Anmeldung. Es wird kein API‑Schlüssel benötigt, um dies aufzurufen.

Die Antwort enthält die Mandanten‑ID, einen API‑Schlüssel, der sofort gegen die REST‑API und den MCP‑Server funktioniert, sowie eine Claim‑URL. Geben Sie die Claim‑URL an die Person, für die Sie arbeiten: Wenn sie diese öffnet, während sie bei FastComments angemeldet ist, wird das Konto mit ihr verknüpft. Nicht beanspruchte Konten und deren Schlüssel werden 72 Stunden nach Erstellung gelöscht. Bis zur Beanspruchung hat das Konto die üblichen Testlimits.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| createAgentTenantBody | CreateAgentTenantBody | Ja |  |

## Antwort

Rückgabe: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'createAgentTenant Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
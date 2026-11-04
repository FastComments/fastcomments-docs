Crea un nuovo account di prova per un agente AI senza una registrazione umana. Non è necessaria alcuna API key per chiamare questo endpoint.

La risposta contiene il tenant id, una API key che funziona immediatamente con la REST API e il server MCP, e un claim URL. Fornisci il claim URL alla persona per cui stai lavorando: aprendolo mentre si è connessi a FastComments, l'account viene associato a lei. Gli account non rivendicati, e le loro chiavi, vengono eliminati 72 ore dopo la creazione. Fino alla rivendicazione, l'account ha i limiti standard della versione di prova.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Sì |  |

## Risposta

Restituisce: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // opzionale
  planId: 3 // opzionale
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
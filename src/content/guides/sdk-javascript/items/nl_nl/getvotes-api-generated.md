## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Antwoord

Retourneert: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getVotes Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-8f3b';
const refCampaign: string | undefined = 'newsletter-march2026'; // optionele queryparameter
const urlId: string = `https://www.example.com/articles/2026/03/25/fastcomments-integration${refCampaign ? `?ref=${refCampaign}` : ''}`;

const votes: GetVotes200Response = await getVotes(tenantId, urlId);
[inline-code-end]

---
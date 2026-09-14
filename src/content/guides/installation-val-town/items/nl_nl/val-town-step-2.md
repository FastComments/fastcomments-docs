`tenantId: "demo"` is een gedeelde openbare sandbox. Het werkt zonder aanmelding, daarom gebruiken de voorbeelden het, maar iedereen die FastComments probeert, schrijft in dezelfde threads en iedereen kan ze modereren. Schakel over voordat je iets publiceert waar je om geeft.

Je tenant‑ID staat op de [API secret page](https://fastcomments.com/auth/my-account/api-secret).

Een tenant‑ID is openbaar en hoort in de browsercode te staan. Een API secret is dat niet, en niets op deze pagina heeft er een nodig.

## Read it from an environment variable

Val Town vals zijn openbaar op het gratis niveau, dus hun bron is wereldwijd leesbaar. Houd alles wat gevoelig is in omgevingsvariabelen, lees ze met `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Dit is belangrijker dan normaal op Val Town om een tweede reden: **het remixen van een val kopieert de sleutels van omgevingsvariabelen, maar niet hun waarden.** Een geheim dat in een omgevingsvariabele wordt bewaard, volgt je val niet naar het account van iemand anders. Een geheim dat in een bestand wordt geschreven, wel.

Terugvallen op `"demo"` houdt de val werkend voor iedereen die deze remixt voordat ze hun eigen tenant instellen.

## EU accounts

Een account, de bijbehorende gegevens en sleutels bevinden zich in één regio. Als die van jou is aangemaakt op `eu.fastcomments.com`, moet elke widget config ook `region: "eu"` bevatten, en worden de scripts geladen van `cdn-eu.fastcomments.com`. Laat anders beide ongewijzigd.
`tenantId: "demo"` is een gedeelde openbare sandbox. Het werkt zonder registratie, daarom gebruiken de voorbeelden het, maar iedereen die FastComments uitprobeert schrijft in dezelfde threads en iedereen kan ze modereren. Wissel vóór je iets publiceert dat je belangrijk vindt.

Je tenant ID staat op de [API secret pagina](https://fastcomments.com/auth/my-account/api-secret).

Een tenant ID is openbaar en hoort in browsercode thuis. Een API secret niet, en niets op deze pagina heeft er een nodig.

## Lees het uit een omgevingsvariabele

Val Town vals zijn openbaar op het gratis tier, dus hun broncode is voor iedereen leesbaar. Bewaar gevoelige informatie in omgevingsvariabelen en lees ze uit met `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Dit is op Val Town om een tweede reden extra belangrijk: **het remixen van een val kopieert de sleutels van omgevingsvariabelen, maar niet hun waarden.** Een secret dat in een omgevingsvariabele staat, volgt je val niet naar iemand anders zijn account. Een secret dat in een bestand is geschreven, wel.

De fallback naar `"demo"` houdt de val werkend voor iedereen die hem remixt voordat die zijn eigen tenant heeft ingesteld.

## EU-accounts

Een account, zijn data en zijn sleutels leven in één regio. Als die van jou is aangemaakt op `eu.fastcomments.com`, heeft elke widget-config ook `region: "eu"` nodig, en worden de scripts geladen vanaf `cdn-eu.fastcomments.com`. Laat in andere gevallen beide met rust.
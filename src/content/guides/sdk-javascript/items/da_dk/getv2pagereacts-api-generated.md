## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## Eksempel

[inline-code-attrs-start title = 'getV2PageReacts Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-42";
const urlId: string = "article-9876";
const ssoToken: string = "sso-abc123";

const reactsWithSso: GetV2PageReacts = await getV2PageReacts(tenantId, urlId, ssoToken);
const reactsWithoutSso: GetV2PageReacts = await getV2PageReacts(tenantId, urlId);
[inline-code-end]
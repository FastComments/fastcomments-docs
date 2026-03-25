---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nej |  |

## Svar

Returnerer: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerators200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getModerators Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-12345-prod';
const moderatorsPage1: GetModerators200Response = await getModerators(tenantId);
const moderatorsPage2: GetModerators200Response = await getModerators(tenantId, 50);
[inline-code-end]

---
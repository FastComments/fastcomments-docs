---
## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getModerator Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media-58';
const id: string = 'mod-82f3b9c1';
const moderatorResponse: GetModerator200Response = await getModerator(tenantId, id);
[inline-code-end]

---
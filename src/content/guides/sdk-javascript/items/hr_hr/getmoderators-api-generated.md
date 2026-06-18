## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerators200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_0a1b2c3d';
const moderators: GetModerators200Response = await getModerators(tenantId);
const skip: number = 20;
const moderatorsPage2: GetModerators200Response = await getModerators(tenantId, skip);
[inline-code-end]

---
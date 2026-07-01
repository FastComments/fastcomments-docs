## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // offset di paginazione opzionale
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // Esempio senza paginazione:
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]
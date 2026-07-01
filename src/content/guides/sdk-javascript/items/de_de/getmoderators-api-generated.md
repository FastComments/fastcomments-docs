## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|----------------|
| tenantId | string | Ja |  |
| skip | number | Nein |  |

## Antwort

Rückgabe: [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'getModerators Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // optionaler Paginierungs-Offset
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // Beispiel ohne Paginierung:
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Svar

Returnerer: [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## Eksempel

[inline-code-attrs-start title = 'getModerators Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // valgfri pagineringsforskydning
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // Eksempel uden paginering:
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]

---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // opcjonalny offset paginacji
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // Przykład bez paginacji:
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]

---
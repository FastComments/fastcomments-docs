## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme-tenant-98765";
  const moderatorsResponse: GetModeratorsResponse = await getModerators(tenantId);
  const skip: number = 25;
  const pagedResponse: GetModeratorsResponse = await getModerators(tenantId, skip);
  console.log(moderatorsResponse, pagedResponse);
}
run();
[inline-code-end]

---
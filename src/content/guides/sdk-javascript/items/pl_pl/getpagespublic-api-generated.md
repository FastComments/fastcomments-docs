Wyświetla listę stron dla najemcy. Używane przez klienta desktopowego FChat do wypełniania listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozwiązanej konfiguracji niestandardowej dla każdej strony.
Strony wymagające SSO są filtrowane na podstawie dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| cursor | string | Nie |  |
| limit | number | Nie |  |
| q | string | Nie |  |
| sortBy | PagesSortBy | Nie |  |
| hasComments | boolean | Nie |  |

## Odpowiedź

Zwraca: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---
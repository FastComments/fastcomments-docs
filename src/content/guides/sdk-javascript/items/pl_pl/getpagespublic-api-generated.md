Wyświetla listę stron dla tenanta. Używane przez klienta desktopowego FChat do wypełnienia listy pokoi. Wymaga, aby `enableFChat` było ustawione na true w rozwiązywanej niestandardowej konfiguracji dla każdej strony. Strony wymagające SSO są filtrowane w oparciu o dostęp grupowy użytkownika wysyłającego żądanie.

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

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---
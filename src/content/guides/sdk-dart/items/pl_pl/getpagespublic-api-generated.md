List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Tak |  |
| cursor | string | query | Nie | Nieprzezroczysty wskaźnik paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | Nie | 1..200, domyślnie 50 |
| q | string | query | Nie | Opcjonalny filtr prefiksu tytułu bez rozróżniania wielkości liter. |
| sortBy | string | query | Nie | Kolejność sortowania. `updatedAt` (domyślnie najnowsze najpierw), `commentCount` (najpierw najwięcej komentarzy), lub `title` (alfabetycznie). |
| hasComments | boolean | query | Nie | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem. |

## Response

Zwraca: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'getPagesPublic Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Nieprzezroczysty wskaźnik paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`.
final limit = 56; // int | 1..200, domyślnie 50
final q = q_example; // String | Opcjonalny filtr prefiksu tytułu bez rozróżniania wielkości liter.
final sortBy = ; // PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie najnowsze najpierw), `commentCount` (najpierw najwięcej komentarzy), lub `title` (alfabetycznie).
final hasComments = true; // bool | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]
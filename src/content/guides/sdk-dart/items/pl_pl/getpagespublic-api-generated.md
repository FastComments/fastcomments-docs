Wyświetla listę stron dla tenanta. Używane przez klienta desktopowego FChat do wypełnienia listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozwiązanej niestandardowej konfiguracji dla każdej strony.
Strony wymagające SSO są filtrowane względem dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| cursor | string | query | Nie | Nieprzezroczysty kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | Nie | 1..200, domyślnie 50 |
| q | string | query | Nie | Opcjonalny filtr prefiksu tytułu, niewrażliwy na wielkość liter. |
| sortBy | string | query | Nie | Kolejność sortowania. `updatedAt` (domyślnie, od najnowszych), `commentCount` (najpierw najwięcej komentarzy) lub `title` (alfabetycznie). |
| hasComments | boolean | query | Nie | Jeśli true, zwraca tylko strony z co najmniej jednym komentarzem. |

## Odpowiedź

Zwraca: `GetPublicPagesResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Nieprzezroczysty kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`.
final limit = 56; // int | 1..200, domyślnie 50
final q = q_example; // String | Opcjonalny filtr prefiksu tytułu, niewrażliwy na wielkość liter.
final sortBy = ; // PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, od najnowszych), `commentCount` (najpierw najwięcej komentarzy) lub `title` (alfabetycznie).
final hasComments = true; // bool | Jeśli true, zwraca tylko strony z co najmniej jednym komentarzem.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]
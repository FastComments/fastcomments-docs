Zbiorcze informacje o użytkownikach dla tenanta. Dla podanych userIds zwraca informacje wyświetlane z User / SSOUser.
Używane przez widżet komentarzy do wzbogacenia użytkowników, którzy właśnie pojawili się przez zdarzenie presence.
Brak kontekstu strony: prywatność jest egzekwowana jednolicie (prywatne profile są maskowane).

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| ids | string | query | Tak | Identyfikatory userIds rozdzielone przecinkami. |

## Odpowiedź

Zwraca: `PageUsersInfoResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Identyfikatory userIds rozdzielone przecinkami.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|--------------|----------|------|
| tenantId | string | query | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: `GetBannedUsersCountResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład getCounts'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCounts(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getCounts: $e\n');
}
[inline-code-end]
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | zapytanie | Tak |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: `GetUserNotificationCountResponse`

## Przykład

[inline-code-attrs-start title = 'getUserNotificationCount Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserNotificationCount(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserNotificationCount: $e\n');
}
[inline-code-end]
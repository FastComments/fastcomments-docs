## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Tak |  |
| badgesUserId | string | query | Nie |  |
| commentId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: `GetUserManualBadgesResponse`

## Przykład

[inline-code-attrs-start title = 'getManualBadgesForUser Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final badgesUserId = badgesUserId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getManualBadgesForUser(tenantId, GetManualBadgesForUserOptions(badgesUserId: badgesUserId, commentId: commentId, sso: sso));
    print(result);
} catch (e) {
    print('Wyjątek podczas wywoływania ModerationApi->getManualBadgesForUser: $e\n');
}
[inline-code-end]
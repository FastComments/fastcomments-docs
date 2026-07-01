## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| value | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: `ModerationUserSearchResponse`

## Primjer

[inline-code-attrs-start title = 'getSearchUsers Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final value = value_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchUsers(tenantId, GetSearchUsersOptions(value: value, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchUsers: $e\n');
}
[inline-code-end]

---
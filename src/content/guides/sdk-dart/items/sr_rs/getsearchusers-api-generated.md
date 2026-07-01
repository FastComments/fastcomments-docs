## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | upit | Da |  |
| value | string | upit | Ne |  |
| sso | string | upit | Ne |  |

## Odgovor

Vraća: `ModerationUserSearchResponse`

## Primer

[inline-code-attrs-start title = 'getSearchUsers Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final value = value_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchUsers(tenantId, GetSearchUsersOptions(value: value, sso: sSO));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchUsers: $e\n');
}
[inline-code-end]
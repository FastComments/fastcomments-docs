## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| value | string | query | No |  |
| sso | string | query | No |  |

## Απάντηση

Επιστρέφει: `ModerationUserSearchResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSearchUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
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
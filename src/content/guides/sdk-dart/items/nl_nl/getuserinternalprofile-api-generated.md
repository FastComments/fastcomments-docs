## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Returns: `GetUserInternalProfileResponse`

## Example

[inline-code-attrs-start title = 'getUserInternalProfile Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserInternalProfile(tenantId, GetUserInternalProfileOptions(commentId: commentId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getUserInternalProfile: $e\n');
}
[inline-code-end]
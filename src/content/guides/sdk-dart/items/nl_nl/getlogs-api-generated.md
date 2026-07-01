## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|---------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| sso | string | query | Nee |  |

## Response

Retourneert: `ModerationAPIGetLogsResponse`

## Example

[inline-code-attrs-start title = 'getLogs Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getLogs(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getLogs: $e\n');
}
[inline-code-end]
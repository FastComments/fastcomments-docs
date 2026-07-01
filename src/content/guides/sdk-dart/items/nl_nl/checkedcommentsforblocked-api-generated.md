## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentIds | string | query | Ja | Een door komma's gescheiden lijst met commentaar-id's. |
| sso | string | query | Nee |  |

## Respons

Retourneert: `CheckBlockedCommentsResponse`

## Voorbeeld

[inline-code-attrs-start title = 'checkedCommentsForBlocked voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Een door komma's gescheiden lijst met commentaar-id's.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]
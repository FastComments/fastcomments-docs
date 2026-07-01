## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|---------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| sso | string | query | Nee |  |

## Respons

Returns: `APIEmptyResponse`

## Voorbeeld

[inline-code-attrs-start title = 'putCloseThread Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putCloseThread(tenantId, urlId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putCloseThread: $e\n');
}
[inline-code-end]
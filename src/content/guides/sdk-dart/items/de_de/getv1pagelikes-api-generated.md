## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |

## Antwort

Rückgabe: `GetV1PageLikes`

## Beispiel

[inline-code-attrs-start title = 'getV1PageLikes Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.getV1PageLikes(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getV1PageLikes: $e\n');
}
[inline-code-end]
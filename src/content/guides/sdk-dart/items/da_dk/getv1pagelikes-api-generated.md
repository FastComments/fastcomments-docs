## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |

## Respons

Returns: `GetV1PageLikes`

## Eksempel

[inline-code-attrs-start title = 'getV1PageLikes Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
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
## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |

## Odgovor

Vrne: `GetV1PageLikes`

## Primer

[inline-code-attrs-start title = 'getV1PageLikes Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
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
## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |

## Odgovor

Vraća: `GetV2PageReacts`

## Primer

[inline-code-attrs-start title = 'getV2PageReacts Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.getV2PageReacts(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getV2PageReacts: $e\n');
}
[inline-code-end]
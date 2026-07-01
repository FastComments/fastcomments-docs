## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |

## Odgovor

Vraća: `CreateV1PageReact`

## Primer

[inline-code-attrs-start title = 'deleteV1PageReact Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.deleteV1PageReact(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteV1PageReact: $e\n');
}
[inline-code-end]
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| id | string | query | Tak |  |

## Odpowiedź

Zwraca: `CreateV1PageReact`

## Przykład

[inline-code-attrs-start title = 'deleteV2PageReact Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteV2PageReact(tenantId, urlId, id);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteV2PageReact: $e\n');
}
[inline-code-end]
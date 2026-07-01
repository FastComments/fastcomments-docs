## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |

## Risposta

Restituisce: `GetV2PageReacts`

## Esempio

[inline-code-attrs-start title = 'Esempio getV2PageReacts'; type = ''; isFunctional = false; inline-code-attrs-end]
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
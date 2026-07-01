## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| urlId | string | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: `APIEmptyResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio putReopenThread'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putReopenThread(tenantId, urlId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putReopenThread: $e\n');
}
[inline-code-end]
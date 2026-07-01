## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | path | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: `ModerationAPIGetLogsResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio getLogs'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getLogs(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getLogs: $e\n');
}
[inline-code-end]
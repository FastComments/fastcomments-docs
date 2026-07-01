## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| commentId | string | path | Sì |  |
| broadcastId | string | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: `ChangeCommentPinStatusResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio pinComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.pinComment(tenantId, commentId, broadcastId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->pinComment: $e\n');
}
[inline-code-end]
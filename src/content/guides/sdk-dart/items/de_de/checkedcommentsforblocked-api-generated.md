## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentIds | string | query | Ja | Eine kommagetrennte Liste von Kommentar-IDs. |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: `CheckBlockedCommentsResponse`

## Beispiel

[inline-code-attrs-start title = 'checkedCommentsForBlocked Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Eine kommagetrennte Liste von Kommentar-IDs.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]
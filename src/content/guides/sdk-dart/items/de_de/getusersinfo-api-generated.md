Bulk-Benutzerinformationen für einen Mandanten. Anhand von userIds werden Anzeigeninformationen aus User / SSOUser zurückgegeben.  
Wird vom Kommentar‑Widget verwendet, um Benutzer zu ergänzen, die gerade über ein Präsenzereignis erschienen sind.  
Kein Seitenkontext: Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|--------------|
| tenantId | string | path | Ja |  |
| ids | string | query | Ja | Kommagetrennte userIds. |

## Antwort

Rückgabe: `PageUsersInfoResponse`

## Beispiel

[inline-code-attrs-start title = 'getUsersInfo Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Kommagetrennte userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]
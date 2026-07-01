## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | query | Ja |  |
| includeByUserIdAndEmail | boolean | query | Nein |  |
| includeByIP | boolean | query | Nein |  |
| includeByEmailDomain | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: `BulkPreBanSummary`

## Beispiel

[inline-code-attrs-start title = 'postBulkPreBanSummary Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final bulkPreBanParams = BulkPreBanParams(); // BulkPreBanParams | 
final includeByUserIdAndEmail = true; // bool | 
final includeByIP = true; // bool | 
final includeByEmailDomain = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postBulkPreBanSummary(tenantId, bulkPreBanParams, PostBulkPreBanSummaryOptions(includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postBulkPreBanSummary: $e\n');
}
[inline-code-end]

---
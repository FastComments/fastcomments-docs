## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| includeByUserIdAndEmail | boolean | query | Ne |  |
| includeByIP | boolean | query | Ne |  |
| includeByEmailDomain | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: `PreBanSummary`

## Primjer

[inline-code-attrs-start title = 'getPreBanSummary Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final includeByUserIdAndEmail = true; // bool | 
final includeByIP = true; // bool | 
final includeByEmailDomain = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getPreBanSummary(tenantId, commentId, GetPreBanSummaryOptions(includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getPreBanSummary: $e\n');
}
[inline-code-end]
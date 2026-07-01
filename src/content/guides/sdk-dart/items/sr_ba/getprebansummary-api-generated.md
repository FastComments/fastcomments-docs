## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Odgovor

Returns: `PreBanSummary`

## Primjer

[inline-code-attrs-start title = 'Primjer getPreBanSummary'; type = ''; isFunctional = false; inline-code-attrs-end]
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
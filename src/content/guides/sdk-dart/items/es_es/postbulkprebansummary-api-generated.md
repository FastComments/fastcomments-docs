## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: `BulkPreBanSummary`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo postBulkPreBanSummary'; type = ''; isFunctional = false; inline-code-attrs-end]
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
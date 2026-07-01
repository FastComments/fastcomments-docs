## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## תגובה

מחזיר: `PatchDomainConfigResponse`

## דוגמה

[inline-code-attrs-start title = 'patchDomainConfig דוגמה'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO קונפיגורציית הרשאת מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domainToUpdate = domainToUpdate_example; // String | 
final patchDomainConfigParams = PatchDomainConfigParams(); // PatchDomainConfigParams | 

try {
    final result = api_instance.patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchDomainConfig: $e\n');
}
[inline-code-end]

---
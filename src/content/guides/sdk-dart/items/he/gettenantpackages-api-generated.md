## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Response

מחזיר: `GetTenantPackagesResponse`

## Example

[inline-code-attrs-start title = 'דוגמה getTenantPackages'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO קונפיגור את האימות למפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל הערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה‑API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantPackages(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantPackages: $e\n');
}
[inline-code-end]
## פרמטרים

| שם | סוג | מיקום | דרוש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

Returns: `APIEmptyResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמה deleteTenantPackage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO הגדר הרשאת מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה‑API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteTenantPackage(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTenantPackage: $e\n');
}
[inline-code-end]
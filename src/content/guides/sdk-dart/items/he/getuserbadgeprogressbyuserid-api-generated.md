---
## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | path | Yes |  |

## תגובה

מחזיר: `APIGetUserBadgeProgressResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserBadgeProgressByUserId'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO: הגדר אישור מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// הסירו את ההערה למטה כדי להגדיר קידומת (לדוגמה, Bearer) למפתח ה-API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressByUserId(tenantId, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressByUserId: $e\n');
}
[inline-code-end]

---
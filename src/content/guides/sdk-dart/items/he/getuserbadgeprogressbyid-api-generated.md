## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## תגובה

מחזיר: `APIGetUserBadgeProgressResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמה לgetUserBadgeProgressById'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO קונפיגורציית הרשאת מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם צריך
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressById(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressById: $e\n');
}
[inline-code-end]
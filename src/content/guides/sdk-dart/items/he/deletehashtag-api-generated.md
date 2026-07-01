## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| tag | string | path | Yes |  |

## תגובה

מחזיר: `APIEmptyResponse`

## דוגמה

[inline-code-attrs-start title = 'deleteHashTag דוגמה'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO הגדר הרשאת מפתח API: api_key
// קבע את מפתח ה-API ל-'YOUR_API_KEY';
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, במידת הצורך
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final deleteHashTagRequestBody = DeleteHashTagRequestBody(); // DeleteHashTagRequestBody | 

try {
    final result = api_instance.deleteHashTag(tenantId, tag, deleteHashTagRequestBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteHashTag: $e\n');
}
[inline-code-end]
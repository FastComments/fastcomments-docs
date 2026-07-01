## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## תגובה

מחזיר: `DeleteSSOUserAPIResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteSSOUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO הגדר הרשאת מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final deleteComments = true; // bool | 
final commentDeleteMode = commentDeleteMode_example; // String | 

try {
    final result = api_instance.deleteSSOUser(tenantId, id, DeleteSSOUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteSSOUser: $e\n');
}
[inline-code-end]
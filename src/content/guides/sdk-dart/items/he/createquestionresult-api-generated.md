## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: `CreateQuestionResultResponse`

## דוגמה

[inline-code-attrs-start title = 'createQuestionResult דוגמה'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO קבע הרשאה של מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל הערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionResultBody = CreateQuestionResultBody(); // CreateQuestionResultBody | 

try {
    final result = api_instance.createQuestionResult(tenantId, createQuestionResultBody);
    print(result);
} catch (e) {
    print('שגיאה בעת קריאה לDefaultApi->createQuestionResult: $e\n');
}
[inline-code-end]
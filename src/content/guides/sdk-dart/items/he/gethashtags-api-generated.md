## Parameters

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| page | number | query | לא |  |

## Response

מחזיר: `GetHashTagsResponse`

## Example

[inline-code-attrs-start title = 'דוגמה getHashTags'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO הגדר אימות מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// הבטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final page = 1.2; // double | 

try {
    final result = api_instance.getHashTags(tenantId, page);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getHashTags: $e\n');
}
[inline-code-end]
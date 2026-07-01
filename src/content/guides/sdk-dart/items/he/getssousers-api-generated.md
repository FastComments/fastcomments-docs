## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| skip | integer | query | לא |  |

## Response

מחזיר: `GetSSOUsersResponse`

## Example

[inline-code-attrs-start title = 'getSSOUsers דוגמה'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO הגדרת הרשאת מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 56; // int | 

try {
    final result = api_instance.getSSOUsers(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUsers: $e\n');
}
[inline-code-end]
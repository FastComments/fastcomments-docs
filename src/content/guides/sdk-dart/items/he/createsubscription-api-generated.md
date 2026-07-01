## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## Response

Returns: `CreateSubscriptionAPIResponse`

## Example

[inline-code-attrs-start title = 'דוגמת createSubscription'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO הגדר הרשאה של מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPIUserSubscriptionData = CreateAPIUserSubscriptionData(); // CreateAPIUserSubscriptionData | 

try {
    final result = api_instance.createSubscription(tenantId, createAPIUserSubscriptionData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createSubscription: $e\n');
}
[inline-code-end]
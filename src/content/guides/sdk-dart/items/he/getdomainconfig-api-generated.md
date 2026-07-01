## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| domain | string | path | כן |  |

## תגובה

מחזיר: `GetDomainConfigResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת getDomainConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO לקבוע אימות מפתח API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל את ההערה שלמטה כדי להגדיר קידומת (לדוגמה Bearer) למפתח API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domain = domain_example; // String | 

try {
    final result = api_instance.getDomainConfig(tenantId, domain);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getDomainConfig: $e\n');
}
[inline-code-end]
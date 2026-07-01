## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| yearNumber | number | query | לא |  |
| monthNumber | number | query | לא |  |
| dayNumber | number | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: `GetTenantDailyUsagesResponse`

## דוגמה

[inline-code-attrs-start title = 'getTenantDailyUsages דוגמה'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO קבע את הרשאת מפתח ה‑API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// בטל את ההערה למטה כדי להגדיר קידומת (לדוגמה, Bearer) למפתח ה‑API, אם נדרש
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final yearNumber = 1.2; // double | 
final monthNumber = 1.2; // double | 
final dayNumber = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantDailyUsages(tenantId, GetTenantDailyUsagesOptions(yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantDailyUsages: $e\n');
}
[inline-code-end]
## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Response

返却: `GetTenantUsersResponse`

## Example

[inline-code-attrs-start title = 'getTenantUsers の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定してください: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 以下のコメントを解除して、必要に応じて API キーのプレフィックス（例: Bearer）を設定してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantUsers(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantUsers: $e\n');
}
[inline-code-end]
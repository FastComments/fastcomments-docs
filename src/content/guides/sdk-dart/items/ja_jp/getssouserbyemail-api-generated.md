## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| email | string | path | はい |  |

## Response

返却: `GetSSOUserByEmailAPIResponse`

## Example

[inline-code-attrs-start title = 'getSSOUserByEmail の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 以下のコメントを解除して、必要に応じてAPIキーのプレフィックス（例: Bearer）を設定
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final email = email_example; // String | 

try {
    final result = api_instance.getSSOUserByEmail(tenantId, email);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUserByEmail: $e\n');
}
[inline-code-end]
## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | integer | query | No |  |

## 回應

Returns: `GetSSOUsersResponse`

## 範例

[inline-code-attrs-start title = 'getSSOUsers 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 金鑰授權: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解以下以設定前綴 (例如 Bearer) 給 API 金鑰，如有需要
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

---
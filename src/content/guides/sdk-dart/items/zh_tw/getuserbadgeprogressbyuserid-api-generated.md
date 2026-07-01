## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | path | Yes |  |

## 回應

Returns: `APIGetUserBadgeProgressResponse`

## 範例

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 金鑰授權: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解以下以設置前綴（例如 Bearer）給 API 金鑰，如有需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressByUserId(tenantId, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressByUserId: $e\n');
}
[inline-code-end]

---
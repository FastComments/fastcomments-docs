## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## 响应

Returns: `APIGetUserBadgeProgressListResponse`

## 示例

[inline-code-attrs-start title = 'getUserBadgeProgressList 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下代码以设置前缀（例如 Bearer）用于 API 密钥（如有需要）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getUserBadgeProgressList(tenantId, GetUserBadgeProgressListOptions(userId: userId, limit: limit, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressList: $e\n');
}
[inline-code-end]
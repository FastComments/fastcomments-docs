## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回：`APIGetUserBadgeProgressResponse`

## 示例

[inline-code-attrs-start title = 'getUserBadgeProgressById 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权：api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下以设置 API 密钥前缀（例如 Bearer），如果需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressById(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressById: $e\n');
}
[inline-code-end]
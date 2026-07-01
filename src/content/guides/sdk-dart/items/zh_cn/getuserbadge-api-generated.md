## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## 响应

返回： `APIGetUserBadgeResponse`

## 示例

[inline-code-attrs-start title = 'getUserBadge 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下内容以设置前缀（例如 Bearer）用于 API 密钥（如有需要）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUserBadge(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadge: $e\n');
}
[inline-code-end]

---
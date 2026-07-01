## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | string | query | No |  |
| commentDeleteMode | string | query | No |  |

## 响应

Returns: `APIEmptyResponse`

## 示例

[inline-code-attrs-start title = 'deleteTenantUser 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下行以设置前缀（例如 Bearer）用于 API 密钥（如有需要）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final deleteComments = deleteComments_example; // String | 
final commentDeleteMode = commentDeleteMode_example; // String | 

try {
    final result = api_instance.deleteTenantUser(tenantId, id, DeleteTenantUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTenantUser: $e\n');
}
[inline-code-end]
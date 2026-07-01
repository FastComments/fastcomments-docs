## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| editKey | string | query | 否 |  |

## 响应

返回: `VoteDeleteResponse`

## 示例

[inline-code-attrs-start title = 'deleteVote 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下行以设置前缀（例如 Bearer）用于 API 密钥，如有需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final editKey = editKey_example; // String | 

try {
    final result = api_instance.deleteVote(tenantId, id, editKey);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteVote: $e\n');
}
[inline-code-end]
## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| deleteComments | boolean | query | 否 |  |
| commentDeleteMode | string | query | 否 |  |

## 响应

返回： `DeleteSSOUserAPIResponse`

## 示例

[inline-code-attrs-start title = 'deleteSSOUser 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释下面以设置前缀（例如 Bearer）用于 API 密钥（如有需要）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final deleteComments = true; // bool | 
final commentDeleteMode = commentDeleteMode_example; // String | 

try {
    final result = api_instance.deleteSSOUser(tenantId, id, DeleteSSOUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteSSOUser: $e\n');
}
[inline-code-end]

---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回： `GetSSOUserByIdAPIResponse`

## 示例

[inline-code-attrs-start title = 'getSSOUserById 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释下面以设置前缀（例如 Bearer）用于 API 密钥（如有需要）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getSSOUserById(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUserById: $e\n');
}
[inline-code-end]
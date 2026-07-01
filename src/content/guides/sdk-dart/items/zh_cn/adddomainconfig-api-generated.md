## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 响应

返回: `AddDomainConfigResponse`

## 示例

[inline-code-attrs-start title = 'addDomainConfig 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下内容以为 API 密钥设置前缀（例如 Bearer），如果需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final addDomainConfigParams = AddDomainConfigParams(); // AddDomainConfigParams | 

try {
    final result = api_instance.addDomainConfig(tenantId, addDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addDomainConfig: $e\n');
}
[inline-code-end]
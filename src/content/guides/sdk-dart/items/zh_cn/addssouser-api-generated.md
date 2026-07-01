## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 响应

返回： `AddSSOUserAPIResponse`

## 示例

[inline-code-attrs-start title = 'addSSOUser 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下内容以为 API 密钥设置前缀（例如 Bearer），如有需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPISSOUserData = CreateAPISSOUserData(); // CreateAPISSOUserData | 

try {
    final result = api_instance.addSSOUser(tenantId, createAPISSOUserData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addSSOUser: $e\n');
}
[inline-code-end]

---
## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## 响应

Returns: `GetTenantsResponse`

## 示例

[inline-code-attrs-start title = 'getTenants 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final meta = meta_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenants(tenantId, GetTenantsOptions(meta: meta, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenants: $e\n');
}
[inline-code-end]
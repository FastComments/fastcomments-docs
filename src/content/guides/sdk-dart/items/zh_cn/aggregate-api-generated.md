聚合文档，通过对其进行分组（如果提供了 `groupBy`）并应用多个操作来实现。支持不同的操作（例如 sum、countDistinct、avg 等）。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| parentTenantId | string | query | 否 |  |
| includeStats | boolean | query | 否 |  |

## 返回

返回： `AggregateResponse`

## 示例

[inline-code-attrs-start title = '聚合示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final aggregationRequest = AggregationRequest(); // AggregationRequest | 
final parentTenantId = parentTenantId_example; // String | 
final includeStats = true; // bool | 

try {
    final result = api_instance.aggregate(tenantId, aggregationRequest, AggregateOptions(parentTenantId: parentTenantId, includeStats: includeStats));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->aggregate: $e\n');
}
[inline-code-end]
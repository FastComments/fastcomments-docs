## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| limit | number | query | 否 |  |
| skip | number | query | 否 |  |
| order | string | query | 否 |  |
| after | number | query | 否 |  |
| before | number | query | 否 |  |

## 响应

返回: `GetAuditLogsResponse`

## 示例

[inline-code-attrs-start title = 'getAuditLogs 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下内容以设置前缀（例如 Bearer）用于 API 密钥，如有需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 
final order = ; // SORTDIR | 
final after = 1.2; // double | 
final before = 1.2; // double | 

try {
    final result = api_instance.getAuditLogs(tenantId, GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getAuditLogs: $e\n');
}
[inline-code-end]
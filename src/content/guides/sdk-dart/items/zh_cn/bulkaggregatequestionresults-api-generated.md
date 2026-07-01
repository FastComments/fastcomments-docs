## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| forceRecalculate | boolean | query | No |  |

## 响应

返回: `BulkAggregateQuestionResultsResponse`

## 示例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下代码以在需要时为 API 密钥设置前缀（例如 Bearer）
#//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final bulkAggregateQuestionResultsRequest = BulkAggregateQuestionResultsRequest(); // BulkAggregateQuestionResultsRequest | 
final forceRecalculate = true; // bool | 

try {
    final result = api_instance.bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->bulkAggregateQuestionResults: $e\n');
}
[inline-code-end]
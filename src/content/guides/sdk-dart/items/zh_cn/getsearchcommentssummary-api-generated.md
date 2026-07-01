## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| value | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: `ModerationCommentSearchResponse`

## 示例

[inline-code-attrs-start title = 'getSearchCommentsSummary 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final value = value_example; // String | 
final filters = filters_example; // String | 
final searchFilters = searchFilters_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchCommentsSummary(tenantId, GetSearchCommentsSummaryOptions(value: value, filters: filters, searchFilters: searchFilters, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchCommentsSummary: $e\n');
}
[inline-code-end]
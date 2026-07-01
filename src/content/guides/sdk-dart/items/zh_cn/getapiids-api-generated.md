## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| afterId | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

Returns: `ModerationAPIGetCommentIdsResponse`

## 示例

[inline-code-attrs-start title = 'getApiIds 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final textSearch = textSearch_example; // String | 
final byIPFromComment = byIPFromComment_example; // String | 
final filters = filters_example; // String | 
final searchFilters = searchFilters_example; // String | 
final afterId = afterId_example; // String | 
final demo = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getApiIds(tenantId, GetApiIdsOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, afterId: afterId, demo: demo, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getApiIds: $e\n');
}
[inline-code-end]
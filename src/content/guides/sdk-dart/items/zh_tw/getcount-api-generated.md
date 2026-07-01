## 參數

| 名稱 | 類型 | 位置 | 必要 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filter | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳： `ModerationAPICountCommentsResponse`

## 範例

[inline-code-attrs-start title = 'getCount 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final textSearch = textSearch_example; // String | 
final byIPFromComment = byIPFromComment_example; // String | 
final filter = filter_example; // String | 
final searchFilters = searchFilters_example; // String | 
final demo = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCount(tenantId, GetCountOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filter: filter, searchFilters: searchFilters, demo: demo, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getCount: $e\n');
}
[inline-code-end]
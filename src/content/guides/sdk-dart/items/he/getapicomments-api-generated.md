## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| page | number | query | לא |  |
| count | number | query | לא |  |
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sorts | string | query | לא |  |
| demo | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: `ModerationAPIGetCommentsResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת getApiComments'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final page = 1.2; // double | 
final count = 1.2; // double | 
final textSearch = textSearch_example; // String | 
final byIPFromComment = byIPFromComment_example; // String | 
final filters = filters_example; // String | 
final searchFilters = searchFilters_example; // String | 
final sorts = sorts_example; // String | 
final demo = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getApiComments(tenantId, GetApiCommentsOptions(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getApiComments: $e\n');
}
[inline-code-end]
## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| afterId | string | query | לא |  |
| demo | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: `ModerationAPIGetCommentIdsResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמה getApiIds'; type = ''; isFunctional = false; inline-code-attrs-end]
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
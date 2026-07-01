## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filters | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| afterId | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: `ModerationAPIGetCommentIdsResponse`

## Primer

[inline-code-attrs-start title = 'Primer getApiIds'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---
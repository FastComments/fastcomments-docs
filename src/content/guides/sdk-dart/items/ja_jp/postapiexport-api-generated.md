## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| text-search | string | クエリ | いいえ |  |
| byIPFromComment | string | クエリ | いいえ |  |
| filters | string | クエリ | いいえ |  |
| searchFilters | string | クエリ | いいえ |  |
| sorts | string | クエリ | いいえ |  |
| sso | string | クエリ | いいえ |  |

## Response

Returns: `ModerationExportResponse`

## Example

[inline-code-attrs-start title = 'postApiExport 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final textSearch = textSearch_example; // String | 
final byIPFromComment = byIPFromComment_example; // String | 
final filters = filters_example; // String | 
final searchFilters = searchFilters_example; // String | 
final sorts = sorts_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postApiExport(tenantId, PostApiExportOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postApiExport: $e\n');
}
[inline-code-end]
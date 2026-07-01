## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| sorts | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: `ModerationExportResponse`

## Пример

[inline-code-attrs-start title = 'postApiExport Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
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
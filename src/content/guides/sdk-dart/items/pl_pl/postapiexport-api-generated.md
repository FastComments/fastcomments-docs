---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | query | Tak |  |
| text-search | string | query | Nie |  |
| byIPFromComment | string | query | Nie |  |
| filters | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| sorts | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: `ModerationExportResponse`

## Przykład

[inline-code-attrs-start title = 'postApiExport Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
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
    final result = api_instance.postApiExport(tenantId, PostApiExportOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, sso: sno));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postApiExport: $e\n');
}
[inline-code-end]

---
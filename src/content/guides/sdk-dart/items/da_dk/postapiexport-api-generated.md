## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Ja |  |
| text-search | string | query | Nej |  |
| byIPFromComment | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| sorts | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: `ModerationExportResponse`

## Eksempel

[inline-code-attrs-start title = 'postApiExport Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---
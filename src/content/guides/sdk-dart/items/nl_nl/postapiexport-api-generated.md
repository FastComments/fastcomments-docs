## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| text-search | string | query | Nee |  |
| byIPFromComment | string | query | Nee |  |
| filters | string | query | Nee |  |
| searchFilters | string | query | Nee |  |
| sorts | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: `ModerationExportResponse`

## Voorbeeld

[inline-code-attrs-start title = 'postApiExport Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
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
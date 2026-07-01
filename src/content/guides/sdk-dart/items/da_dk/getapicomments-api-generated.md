---
## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | number | query | Nej |  |
| count | number | query | Nej |  |
| text-search | string | query | Nej |  |
| byIPFromComment | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| sorts | string | query | Nej |  |
| demo | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: `ModerationAPIGetCommentsResponse`

## Eksempel

[inline-code-attrs-start title = 'getApiComments Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---
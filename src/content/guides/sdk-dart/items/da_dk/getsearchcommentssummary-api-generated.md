## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| value | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: `ModerationCommentSearchResponse`

## Eksempel

[inline-code-attrs-start title = 'getSearchCommentsSummary Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final value = value_example; // String | 
final filters = filters_example; // String | 
final searchFilters = searchFilters_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchCommentsSummary(tenantId, GetSearchCommentsSummaryOptions(value: value, filters: filters, searchFilters: searchFilters, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchCommentsSummary: $e\n');
}
[inline-code-end]
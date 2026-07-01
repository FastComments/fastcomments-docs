## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| text-search | string | query | Nee |  |
| byIPFromComment | string | query | Nee |  |
| filter | string | query | Nee |  |
| searchFilters | string | query | Nee |  |
| demo | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: `ModerationAPICountCommentsResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getCount Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final textSearch = textSearch_example; // String | 
final byIPFromComment = byIPFromComment_example; // String | 
final filter = filter_example; // String | 
final searchFilters = searchFilters_example; // String | 
final demo = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCount(tenantId, GetCountOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filter: filter, searchFilters: searchFilters, demo: demo, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getCount: $e\n');
}
[inline-code-end]
## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| text-search | string | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Retourneert: `ModerationSuggestResponse`

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getSearchSuggest'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final textSearch = textSearch_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchSuggest(tenantId, GetSearchSuggestOptions(textSearch: textSearch, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchSuggest: $e\n');
}
[inline-code-end]
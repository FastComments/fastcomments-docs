## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| text-search | string | query | Nein |  |
| byIPFromComment | string | query | Nein |  |
| filter | string | query | Nein |  |
| searchFilters | string | query | Nein |  |
| demo | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: `ModerationAPICountCommentsResponse`

## Beispiel

[inline-code-attrs-start title = 'getCount Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
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
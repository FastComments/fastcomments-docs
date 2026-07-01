## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | put | Da |  |
| search | string | upit | Da |  |
| locale | string | upit | Ne |  |
| rating | string | upit | Ne |  |
| page | number | upit | Ne |  |

## Response

Vraća: `GetGifsSearchResponse`

## Primer

[inline-code-attrs-start title = 'getGifsSearch Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final search = search_example; // String | 
final locale = locale_example; // String | 
final rating = rating_example; // String | 
final page = 1.2; // double | 

try {
    final result = api_instance.getGifsSearch(tenantId, search, GetGifsSearchOptions(locale: locale, rating: rating, page: page));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getGifsSearch: $e\n');
}
[inline-code-end]
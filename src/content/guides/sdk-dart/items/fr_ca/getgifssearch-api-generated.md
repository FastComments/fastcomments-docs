## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| search | string | query | Yes |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Response

Retourne : `GetGifsSearchResponse`

## Example

[inline-code-attrs-start title = 'getGifsSearch Exemple'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // Chaîne |
final search = search_example; // Chaîne |
final locale = locale_example; // Chaîne |
final rating = rating_example; // Chaîne |
final page = 1.2; // double |

try {
    final result = api_instance.getGifsSearch(tenantId, search, GetGifsSearchOptions(locale: locale, rating: rating, page: page));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getGifsSearch: $e\n');
}
[inline-code-end]
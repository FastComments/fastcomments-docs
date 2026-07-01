## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | path | Oui |  |
| search | string | query | Oui |  |
| locale | string | query | Non |  |
| rating | string | query | Non |  |
| page | number | query | Non |  |

## Réponse

Retourne : `GetGifsSearchResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getGifsSearch'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---
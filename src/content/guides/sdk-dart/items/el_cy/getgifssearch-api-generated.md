## Παράμετροι

| Όνομα | Τύπο | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|-------------|
| tenantId | string | path | Yes |  |
| search | string | query | Yes |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Απόκριση

Επιστρέφει: `GetGifsSearchResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getGifsSearch'; type = ''; isFunctional = false; inline-code-attrs-end]
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
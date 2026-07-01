## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | path | Yes |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Odpowiedź

Zwraca: `GetGifsTrendingResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład getGifsTrending'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final locale = locale_example; // String | 
final rating = rating_example; // String | 
final page = 1.2; // double | 

try {
    final result = api_instance.getGifsTrending(tenantId, GetGifsTrendingOptions(locale: locale, rating: rating, page: page));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getGifsTrending: $e\n');
}
[inline-code-end]
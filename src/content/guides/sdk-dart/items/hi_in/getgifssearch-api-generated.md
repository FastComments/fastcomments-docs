## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| search | string | query | हाँ |  |
| locale | string | query | नहीं |  |
| rating | string | query | नहीं |  |
| page | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `GetGifsSearchResponse`

## उदाहरण

[inline-code-attrs-start title = 'getGifsSearch उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
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
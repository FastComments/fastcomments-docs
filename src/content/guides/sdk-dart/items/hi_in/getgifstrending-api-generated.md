## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| locale | string | query | नहीं |  |
| rating | string | query | नहीं |  |
| page | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `GetGifsTrendingResponse`

## उदाहरण

[inline-code-attrs-start title = 'getGifsTrending उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
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
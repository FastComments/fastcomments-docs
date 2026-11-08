## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| text-search | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: `ModerationSuggestResponse`

## उदाहरण

[inline-code-attrs-start title = 'getSearchSuggest उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
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
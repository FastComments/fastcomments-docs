## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| text-search | string | query | नहीं |  |
| byIPFromComment | string | query | नहीं |  |
| filter | string | query | नहीं |  |
| searchFilters | string | query | नहीं |  |
| demo | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `ModerationAPICountCommentsResponse`

## उदाहरण

[inline-code-attrs-start title = 'getCount उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
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
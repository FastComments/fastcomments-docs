## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| value | string | query | नहीं |  |
| filters | string | query | नहीं |  |
| searchFilters | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `ModerationCommentSearchResponse`

## उदाहरण

[inline-code-attrs-start title = 'getSearchCommentsSummary उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final value = value_example; // String | 
final filters = filters_example; // String | 
final searchFilters = searchFilters_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchCommentsSummary(tenantId, GetSearchCommentsSummaryOptions(value: value, filters: filters, searchFilters: searchFilters, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchCommentsSummary: $e\n');
}
[inline-code-end]
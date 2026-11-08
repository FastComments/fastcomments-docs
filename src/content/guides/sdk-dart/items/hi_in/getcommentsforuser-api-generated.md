## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| userId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| repliesToUserId | string | query | नहीं |  |
| page | number | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `GetCommentsForUserResponse`

## उदाहरण

[inline-code-attrs-start title = 'getCommentsForUser उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final userId = userId_example; // String | 
final direction = ; // SortDirections | 
final repliesToUserId = repliesToUserId_example; // String | 
final page = 1.2; // double | 
final includei10n = true; // bool | 
final locale = locale_example; // String | 
final isCrawler = true; // bool | 

try {
    final result = api_instance.getCommentsForUser(GetCommentsForUserOptions(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentsForUser: $e\n');
}
[inline-code-end]
## Parameters

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## Response

반환값: `GetCommentsForUserResponse`

## Example

[inline-code-attrs-start title = 'getCommentsForUser 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---
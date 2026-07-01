## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| locale | string | query | 아니오 |  |
| rating | string | query | 아니오 |  |
| page | number | query | 아니오 |  |

## 응답

Returns: `GetGifsTrendingResponse`

## 예시

[inline-code-attrs-start title = 'getGifsTrending 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
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
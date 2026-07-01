## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | パス | はい |  |
| search | string | クエリ | はい |  |
| locale | string | クエリ | いいえ |  |
| rating | string | クエリ | いいえ |  |
| page | number | クエリ | いいえ |  |

## レスポンス

戻り値: `GetGifsSearchResponse`

## 例

[inline-code-attrs-start title = 'getGifsSearch の例'; type = ''; isFunctional = false; inline-code-attrs-end]
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
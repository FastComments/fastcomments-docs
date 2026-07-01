## パラメータ

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| sso | string | query | No |  |

## 応答

戻り値: `ModerationSuggestResponse`

## 例

[inline-code-attrs-start title = 'getSearchSuggest の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // 文字列 | 
final textSearch = textSearch_example; // 文字列 | 
final sso = sso_example; // 文字列 | 

try {
    final result = api_instance.getSearchSuggest(tenantId, GetSearchSuggestOptions(textSearch: textSearch, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchSuggest: $e\n');
}
[inline-code-end]
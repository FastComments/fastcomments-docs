## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|------|-----|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| title | string | query | いいえ |  |

## レスポンス

返り値: `CreateV1PageReact`

## 例

[inline-code-attrs-start title = 'createV1PageReact の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final title = title_example; // String | 

try {
    final result = api_instance.createV1PageReact(tenantId, urlId, title);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createV1PageReact: $e\n');
}
[inline-code-end]
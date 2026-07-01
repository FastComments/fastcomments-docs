## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| commentId | string | path | Evet |  |
| editKey | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: `PublicAPIGetCommentTextResponse`

## Örnek

[inline-code-attrs-start title = 'getCommentText Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCommentText(tenantId, commentId, GetCommentTextOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentText: $e\n');
}
[inline-code-end]
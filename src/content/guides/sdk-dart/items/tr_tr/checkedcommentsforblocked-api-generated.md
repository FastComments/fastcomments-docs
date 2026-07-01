## Parametreler

| Name | Type | Location | Required | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentIds | string | query | Yes | Virgülle ayrılmış yorum kimlikleri listesi. |
| sso | string | query | No |  |

## Yanıt

Döndürür: `CheckBlockedCommentsResponse`

## Örnek

[inline-code-attrs-start title = 'checkedCommentsForBlocked Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Virgülle ayrılmış yorum kimlikleri listesi.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]
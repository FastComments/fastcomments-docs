---
## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'postRestoreDeletedComment Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postRestoreDeletedComment(tenantId, commentId, PostRestoreDeletedCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postRestoreDeletedComment: $e\n');
}
[inline-code-end]

---
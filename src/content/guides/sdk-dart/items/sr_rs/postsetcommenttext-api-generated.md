## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: `SetCommentTextResponse`

## Primer

[inline-code-attrs-start title = 'postSetCommentText Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final setCommentTextParams = SetCommentTextParams(); // SetCommentTextParams | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentText(tenantId, commentId, setCommentTextParams, PostSetCommentTextOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentText: $e\n');
}
[inline-code-end]
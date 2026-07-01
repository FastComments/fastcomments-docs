## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| spam | boolean | query | Ne |  |
| permNotSpam | boolean | query | Ne |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'postSetCommentSpamStatus Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final spam = true; // bool | 
final permNotSpam = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentSpamStatus(tenantId, commentId, PostSetCommentSpamStatusOptions(spam: spam, permNotSpam: permNotSpam, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentSpamStatus: $e\n');
}
[inline-code-end]

---
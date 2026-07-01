## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| approved | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vraća: `SetCommentApprovedResponse`

## Primjer

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final approved = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentApprovalStatus(tenantId, commentId, PostSetCommentApprovalStatusOptions(approved: approved, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentApprovalStatus: $e\n');
}
[inline-code-end]

---
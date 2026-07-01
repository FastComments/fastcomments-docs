## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| commentId | string | path | Tak |  |
| approved | boolean | query | Nie |  |
| broadcastId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Returns: `SetCommentApprovedResponse`

## Przykład

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
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
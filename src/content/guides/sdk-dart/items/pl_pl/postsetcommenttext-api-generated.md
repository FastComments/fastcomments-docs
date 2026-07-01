## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| commentId | string | path | Tak |  |
| broadcastId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: `SetCommentTextResponse`

## Przykład

[inline-code-attrs-start title = 'postSetCommentText Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
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
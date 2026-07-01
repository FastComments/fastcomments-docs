## Параметры

| Имя | Тип | Размещение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| voteId | string | path | Да |  |
| broadcastId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: `VoteDeleteResponse`

## Пример

[inline-code-attrs-start title = 'Пример deleteModerationVote'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final voteId = voteId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteModerationVote(tenantId, commentId, voteId, DeleteModerationVoteOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->deleteModerationVote: $e\n');
}
[inline-code-end]
Сводная информация о пользователях для арендатора. По заданным userIds возвращается отображаемая информация из User / SSOUser.  
Используется виджетом комментариев для обогащения данных о пользователях, которые только что появились через событие присутствия.  
Нет контекста страницы: конфиденциальность применяется единообразно (частные профили скрыты).

## Parameters

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | path | Да |  |
| ids | string | query | Да | Разделённые запятыми userIds. |

## Response

Returns: `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Comma-delimited userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]
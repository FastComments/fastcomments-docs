## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| value | string | query | Нет |  |
| filters | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: `ModerationCommentSearchResponse`

## Пример

[inline-code-attrs-start title = 'getSearchCommentsSummary Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final value = value_example; // String | 
final filters = filters_example; // String | 
final searchFilters = searchFilters_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchCommentsSummary(tenantId, GetSearchCommentsSummaryOptions(value: value, filters: filters, searchFilters: searchFilters, sso: sso));
    print(result);
} catch (e) {
    print('Исключение при вызове ModerationApi->getSearchCommentsSummary: $e\n');
}
[inline-code-end]
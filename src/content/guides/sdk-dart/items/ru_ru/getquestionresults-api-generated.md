## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Нет |  |
| userId | string | query | Нет |  |
| startDate | string | query | Нет |  |
| questionId | string | query | Нет |  |
| questionIds | string | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: `GetQuestionResultsResponse`

## Пример

[inline-code-attrs-start title = 'Пример getQuestionResults'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, при необходимости
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final userId = userId_example; // String | 
final startDate = startDate_example; // String | 
final questionId = questionId_example; // String | 
final questionIds = questionIds_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getQuestionResults(tenantId, GetQuestionResultsOptions(urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionResults: $e\n');
}
[inline-code-end]

---
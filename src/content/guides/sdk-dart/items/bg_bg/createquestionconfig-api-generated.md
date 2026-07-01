## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes |  |

## Отговор

Връща: `CreateQuestionConfigResponse`

## Пример

[inline-code-attrs-start title = 'createQuestionConfig Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте оторизация с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// махнете коментара по-долу, за да зададете представка (напр. Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionConfigBody = CreateQuestionConfigBody(); // CreateQuestionConfigBody | 

try {
    final result = api_instance.createQuestionConfig(tenantId, createQuestionConfigBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createQuestionConfig: $e\n');
}
[inline-code-end]
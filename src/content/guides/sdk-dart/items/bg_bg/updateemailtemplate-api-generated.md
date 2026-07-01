## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'updateEmailTemplate Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте авторизация с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateEmailTemplateBody = UpdateEmailTemplateBody(); // UpdateEmailTemplateBody | 

try {
    final result = api_instance.updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateEmailTemplate: $e\n');
}
[inline-code-end]

---
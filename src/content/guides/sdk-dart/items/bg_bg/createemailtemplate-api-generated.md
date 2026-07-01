## Параметри

| Име | Тип | Локация | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Отговор

Връща: `CreateEmailTemplateResponse`

## Пример

[inline-code-attrs-start title = 'Пример за createEmailTemplate'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте оторизация на API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createEmailTemplateBody = CreateEmailTemplateBody(); // CreateEmailTemplateBody | 

try {
    final result = api_instance.createEmailTemplate(tenantId, createEmailTemplateBody);
    print(result);
} catch (e) {
    print('Изключение при извикване на DefaultApi->createEmailTemplate: $e\n');
}
[inline-code-end]
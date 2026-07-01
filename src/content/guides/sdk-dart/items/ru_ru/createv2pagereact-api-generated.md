## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| id | string | query | Да |  |
| title | string | query | Нет |  |

## Ответ

Возвращает: `CreateV1PageReact`

## Пример

[inline-code-attrs-start title = 'createV2PageReact Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final id = id_example; // String | 
final title = title_example; // String | 

try {
    final result = api_instance.createV2PageReact(tenantId, urlId, id, title);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createV2PageReact: $e\n');
}
[inline-code-end]
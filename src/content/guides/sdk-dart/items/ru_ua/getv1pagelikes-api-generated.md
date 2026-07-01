## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |

## Ответ

Returns: `GetV1PageLikes`

## Пример

[inline-code-attrs-start title = 'Пример getV1PageLikes'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.getV1PageLikes(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getV1PageLikes: $e\n');
}
[inline-code-end]
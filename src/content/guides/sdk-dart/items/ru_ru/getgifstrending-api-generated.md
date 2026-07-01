## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| locale | string | query | Нет |  |
| rating | string | query | Нет |  |
| page | number | query | Нет |  |

## Ответ

Возвращает: `GetGifsTrendingResponse`

## Пример

[inline-code-attrs-start title = 'Пример getGifsTrending'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final locale = locale_example; // String | 
final rating = rating_example; // String | 
final page = 1.2; // double | 

try {
    final result = api_instance.getGifsTrending(tenantId, GetGifsTrendingOptions(locale: locale, rating: rating, page: page));
    print(result);
} catch (e) {
    print('Исключение при вызове PublicApi->getGifsTrending: $e\n');
}
[inline-code-end]
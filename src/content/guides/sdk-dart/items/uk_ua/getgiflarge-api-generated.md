## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| largeInternalURLSanitized | string | query | Yes |  |

## Відповідь

Повертає: `GifGetLargeResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getGifLarge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final largeInternalURLSanitized = largeInternalURLSanitized_example; // String | 

try {
    final result = api_instance.getGifLarge(tenantId, largeInternalURLSanitized);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getGifLarge: $e\n');
}
[inline-code-end]
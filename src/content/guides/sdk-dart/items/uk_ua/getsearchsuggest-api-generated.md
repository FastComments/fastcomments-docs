## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| text-search | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `ModerationSuggestResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getSearchSuggest'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final textSearch = textSearch_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchSuggest(tenantId, GetSearchSuggestOptions(textSearch: textSearch, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchSuggest: $e\n');
}
[inline-code-end]
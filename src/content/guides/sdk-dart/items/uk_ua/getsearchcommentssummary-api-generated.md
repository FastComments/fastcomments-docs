## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| value | string | query | Ні |  |
| filters | string | query | Ні |  |
| searchFilters | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `ModerationCommentSearchResponse`

## Приклад

[inline-code-attrs-start title = 'getSearchCommentsSummary Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
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
    print('Exception when calling ModerationApi->getSearchCommentsSummary: $e\n');
}
[inline-code-end]
## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| text-search | string | query | Ні |  |
| byIPFromComment | string | query | Ні |  |
| filters | string | query | Ні |  |
| searchFilters | string | query | Ні |  |
| afterId | string | query | Ні |  |
| demo | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `ModerationAPIGetCommentIdsResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getApiIds'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final textSearch = textSearch_example; // String | 
final byIPFromComment = byIPFromComment_example; // String | 
final filters = filters_example; // String | 
final searchFilters = searchFilters_example; // String | 
final afterId = afterId_example; // String | 
final demo = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getApiIds(tenantId, GetApiIdsOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, afterId: afterId, demo: demo, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getApiIds: $e\n');
}
[inline-code-end]

---
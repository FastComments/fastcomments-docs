req
tenantId
urlId

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| page | integer | query | Нет |  |
| direction | string | query | Нет |  |
| sso | string | query | Нет |  |
| skip | integer | query | Нет |  |
| skipChildren | integer | query | Нет |  |
| limit | integer | query | Нет |  |
| limitChildren | integer | query | Нет |  |
| countChildren | boolean | query | Нет |  |
| fetchPageForCommentId | string | query | Нет |  |
| includeConfig | boolean | query | Нет |  |
| countAll | boolean | query | Нет |  |
| includei10n | boolean | query | Нет |  |
| locale | string | query | Нет |  |
| modules | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |
| includeNotificationCount | boolean | query | Нет |  |
| asTree | boolean | query | Нет |  |
| maxTreeDepth | integer | query | Нет |  |
| useFullTranslationIds | boolean | query | Нет |  |
| parentId | string | query | Нет |  |
| searchText | string | query | Нет |  |
| hashTags | array | query | Нет |  |
| userId | string | query | Нет |  |
| customConfigStr | string | query | Нет |  |
| afterCommentId | string | query | Нет |  |
| beforeCommentId | string | query | Нет |  |

## Ответ

Возвращает: `GetCommentsResponseWithPresencePublicComment`

## Пример

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final page = 56; // int | 
final direction = ; // SortDirections | 
final sso = sso_example; // String | 
final skip = 56; // int | 
final skipChildren = 56; // int | 
final limit = 56; // int | 
final limitChildren = 56; // int | 
final countChildren = true; // bool | 
final fetchPageForCommentId = fetchPageForCommentId_example; // String | 
final includeConfig = true; // bool | 
final countAll = true; // bool | 
final includei10n = true; // bool | 
final locale = locale_example; // String | 
final modules = modules_example; // String | 
final isCrawler = true; // bool | 
final includeNotificationCount = true; // bool | 
final asTree = true; // bool | 
final maxTreeDepth = 56; // int | 
final useFullTranslationIds = true; // bool | 
final parentId = parentId_example; // String | 
final searchText = searchText_example; // String | 
final hashTags = []; // List<String> | 
final userId = userId_example; // String | 
final customConfigStr = customConfigStr_example; // String | 
final afterCommentId = afterCommentId_example; // String | 
final beforeCommentId = beforeCommentId_example; // String | 

try {
    final result = api_instance.getCommentsPublic(tenantId, urlId, GetCommentsPublicOptions(page: page, direction: direction, sso: sso, skip: skip, skipChildren: skipChildren, limit: limit, limitChildren: limitChildren, countChildren: countChildren, fetchPageForCommentId: fetchPageForCommentId, includeConfig: includeConfig, countAll: countAll, includei10n: includei10n, locale: locale, modules: modules, isCrawler: isCrawler, includeNotificationCount: includeNotificationCount, asTree: asTree, maxTreeDepth: maxTreeDepth, useFullTranslationIds: useFullTranslationIds, parentId: parentId, searchText: searchText, hashTags: hashTags, userId: userId, customConfigStr: customConfigStr, afterCommentId: afterCommentId, beforeCommentId: beforeCommentId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentsPublic: $e\n');
}
[inline-code-end]
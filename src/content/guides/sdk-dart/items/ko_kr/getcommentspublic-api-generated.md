req
tenantId
urlId

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| page | integer | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |
| skip | integer | query | 아니오 |  |
| skipChildren | integer | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| limitChildren | integer | query | 아니오 |  |
| countChildren | boolean | query | 아니오 |  |
| fetchPageForCommentId | string | query | 아니오 |  |
| includeConfig | boolean | query | 아니오 |  |
| countAll | boolean | query | 아니오 |  |
| includei10n | boolean | query | 아니오 |  |
| locale | string | query | 아니오 |  |
| modules | string | query | 아니오 |  |
| isCrawler | boolean | query | 아니오 |  |
| includeNotificationCount | boolean | query | 아니오 |  |
| asTree | boolean | query | 아니오 |  |
| maxTreeDepth | integer | query | 아니오 |  |
| useFullTranslationIds | boolean | query | 아니오 |  |
| parentId | string | query | 아니오 |  |
| searchText | string | query | 아니오 |  |
| hashTags | array | query | 아니오 |  |
| userId | string | query | 아니오 |  |
| customConfigStr | string | query | 아니오 |  |
| afterCommentId | string | query | 아니오 |  |
| beforeCommentId | string | query | 아니오 |  |

## 응답

반환: `GetCommentsResponseWithPresencePublicComment`

## 예시

[inline-code-attrs-start title = 'getCommentsPublic 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---
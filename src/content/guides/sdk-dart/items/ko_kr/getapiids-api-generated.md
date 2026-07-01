## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| afterId | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## 응답

반환: `ModerationAPIGetCommentIdsResponse`

## 예시

[inline-code-attrs-start title = 'getApiIds 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
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
req  
tenantId  
afterId  

## 參數

| 名稱 | 類型 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| afterId | string | query | 否 |  |
| limit | integer | query | 否 |  |
| tags | array | query | 否 |  |
| sso | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeUserInfo | boolean | query | 否 |  |

## 回應

返回： `PublicFeedPostsResponse`

## 範例

[inline-code-attrs-start title = 'getFeedPostsPublic 範例'; type = ''; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final afterId = afterId_example; // String | 
final limit = 56; // int | 
final tags = []; // List<String> | 
final sso = sso_example; // String | 
final isCrawler = true; // bool | 
final includeUserInfo = true; // bool | 

try {
    final result = api_instance.getFeedPostsPublic(tenantId, GetFeedPostsPublicOptions(afterId: afterId, limit: limit, tags: tags, sso: sno, isCrawler: isCrawler, includeUserInfo: includeUserInfo));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getFeedPostsPublic: $e\n');
}
[inline-code-end]
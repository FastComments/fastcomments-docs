Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Used by the comment widget to enrich users that just appeared via a presence event.  
No page context: privacy is enforced uniformly (private profiles are masked).

## Parameters

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|------|------|------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | 以逗號分隔的 userIds。 |

## Response

回傳：`PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | 以逗號分隔的 userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]
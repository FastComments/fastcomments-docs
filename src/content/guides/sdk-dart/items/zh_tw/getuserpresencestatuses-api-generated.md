## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlIdWS | string | query | 是 |  |
| userIds | string | query | 是 |  |

## 回應

返回： `GetUserPresenceStatusesResponse`

## 範例

[inline-code-attrs-start title = 'getUserPresenceStatuses 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlIdWS = urlIdWS_example; // String | 
final userIds = userIds_example; // String | 

try {
    final result = api_instance.getUserPresenceStatuses(tenantId, urlIdWS, userIds);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserPresenceStatuses: $e\n');
}
[inline-code-end]
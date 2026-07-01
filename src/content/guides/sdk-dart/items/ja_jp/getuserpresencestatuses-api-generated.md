## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlIdWS | string | query | Yes |  |
| userIds | string | query | Yes |  |

## レスポンス

返却: `GetUserPresenceStatusesResponse`

## 例

[inline-code-attrs-start title = 'getUserPresenceStatuses の例'; type = ''; isFunctional = false; inline-code-attrs-end]
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
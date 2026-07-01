## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlIdWS | string | query | Yes |  |
| userIds | string | query | Yes |  |

## 응답

반환: `GetUserPresenceStatusesResponse`

## 예시

[inline-code-attrs-start title = 'getUserPresenceStatuses 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
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
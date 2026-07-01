## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## 응답

반환: `GetTenantManualBadgesResponse`

## 예시

[inline-code-attrs-start title = 'getManualBadges 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getManualBadges(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getManualBadges: $e\n');
}
[inline-code-end]
## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: `GetUserTrustFactorResponse`

## 예시

[inline-code-attrs-start title = 'getTrustFactor 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getTrustFactor(tenantId, GetTrustFactorOptions(userId: userId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getTrustFactor: $e\n');
}
[inline-code-end]
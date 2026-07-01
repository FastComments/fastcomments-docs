## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| sso | string | query | No |  |

## レスポンス

返却: `GetUserTrustFactorResponse`

## Example

[inline-code-attrs-start title = 'getTrustFactor の例'; type = ''; isFunctional = false; inline-code-attrs-end]
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
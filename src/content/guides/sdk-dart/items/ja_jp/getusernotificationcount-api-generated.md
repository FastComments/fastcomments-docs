## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: `GetUserNotificationCountResponse`

## 例

[inline-code-attrs-start title = 'getUserNotificationCount 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserNotificationCount(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserNotificationCount: $e\n');
}
[inline-code-end]

---
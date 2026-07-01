## 參數

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|----------|------|-------------|
| tenantId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

返回：`APIModerateGetUserBanPreferencesResponse`

## 範例

[inline-code-attrs-start title = 'getUserBanPreference 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserBanPreference(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getUserBanPreference: $e\n');
}
[inline-code-end]

---
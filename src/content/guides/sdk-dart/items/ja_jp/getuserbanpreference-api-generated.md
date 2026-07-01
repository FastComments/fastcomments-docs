## パラメータ

| 名前 | タイプ | ロケーション | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: `APIModerateGetUserBanPreferencesResponse`

## 例

[inline-code-attrs-start title = 'getUserBanPreference の例'; type = ''; isFunctional = false; inline-code-attrs-end]
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
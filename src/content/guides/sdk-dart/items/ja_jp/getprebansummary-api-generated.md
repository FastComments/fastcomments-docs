## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| includeByUserIdAndEmail | boolean | query | いいえ |  |
| includeByIP | boolean | query | いいえ |  |
| includeByEmailDomain | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: `PreBanSummary`

## 例

[inline-code-attrs-start title = 'getPreBanSummary の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final includeByUserIdAndEmail = true; // bool | 
final includeByIP = true; // bool | 
final includeByEmailDomain = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getPreBanSummary(tenantId, commentId, GetPreBanSummaryOptions(includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getPreBanSummary: $e\n');
}
[inline-code-end]
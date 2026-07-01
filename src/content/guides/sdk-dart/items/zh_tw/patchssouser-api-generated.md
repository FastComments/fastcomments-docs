## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | boolean | query | No |  |

## 回應

返回: `PatchSSOUserAPIResponse`

## 範例

[inline-code-attrs-start title = 'patchSSOUser 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 金鑰授權：api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解以下以設定 API 金鑰的前綴（例如 Bearer），如果需要的話
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String |
final id = id_example; // String |
final updateAPISSOUserData = UpdateAPISSOUserData(); // UpdateAPISSOUserData |
final updateComments = true; // bool |

try {
    final result = api_instance.patchSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchSSOUser: $e\n');
}
[inline-code-end]
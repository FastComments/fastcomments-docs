## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## 応答

返り値: `GetSSOUserByIdAPIResponse`

## 例

[inline-code-attrs-start title = 'getSSOUserById の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 以下のコメントアウトを解除して、必要に応じて API キーのプレフィックス (例: Bearer) を設定
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getSSOUserById(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUserById: $e\n');
}
[inline-code-end]

---
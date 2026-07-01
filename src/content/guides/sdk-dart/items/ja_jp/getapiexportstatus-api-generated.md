## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## 応答

返却: `ModerationExportStatusResponse`

## 例

[inline-code-attrs-start title = 'getApiExportStatus の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final batchJobId = batchJobId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getApiExportStatus(tenantId, GetApiExportStatusOptions(batchJobId: batchJobId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getApiExportStatus: $e\n');
}
[inline-code-end]

---
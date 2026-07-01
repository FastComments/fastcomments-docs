## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| includeByUserIdAndEmail | boolean | query | Нет |  |
| includeByIP | boolean | query | Нет |  |
| includeByEmailDomain | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Returns: `BulkPreBanSummary`

## Пример

[inline-code-attrs-start title = 'Пример postBulkPreBanSummary'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final bulkPreBanParams = BulkPreBanParams(); // BulkPreBanParams | 
final includeByUserIdAndEmail = true; // bool | 
final includeByIP = true; // bool | 
final includeByEmailDomain = true; // bool | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postBulkPreBanSummary(tenantId, bulkPreBanParams, PostBulkPreBanSummaryOptions(includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postBulkPreBanSummary: $e\n');
}
[inline-code-end]
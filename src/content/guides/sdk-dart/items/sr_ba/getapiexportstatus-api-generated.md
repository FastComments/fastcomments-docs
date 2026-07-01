## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| batchJobId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: `ModerationExportStatusResponse`

## Пример

[inline-code-attrs-start title = 'getApiExportStatus Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
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
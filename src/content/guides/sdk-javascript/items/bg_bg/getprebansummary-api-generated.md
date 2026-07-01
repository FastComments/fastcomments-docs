## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| includeByUserIdAndEmail | boolean | Не |  |
| includeByIP | boolean | Не |  |
| includeByEmailDomain | boolean | Не |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`GetPreBanSummaryResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPreBanSummaryResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getPreBanSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "5f8d0c2e4b1a2c3d4e5f6a7b";
const includeByUserIdAndEmail: boolean = true;
const includeByIP: boolean = true;
const includeByEmailDomain: boolean = false;
const tenantId: string = "tenant-001";
const sso: string = "sso-xyz-123";

const preBanSummary: GetPreBanSummaryResponse = await getPreBanSummary(
  commentId,
  includeByUserIdAndEmail,
  includeByIP,
  includeByEmailDomain,
  tenantId,
  sso
);
[inline-code-end]
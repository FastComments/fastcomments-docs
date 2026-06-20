## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Да |  |
| includeByUserIdAndEmail | boolean | Не |  |
| includeByIP | boolean | Не |  |
| includeByEmailDomain | boolean | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`PreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PreBanSummary.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getPreBanSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt-9f7b2e3d-45a1';
const includeByUserIdAndEmail: boolean = true;
const includeByIP: boolean = true;
const includeByEmailDomain: boolean = false;
const sso: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';

const summary: PreBanSummary = await getPreBanSummary(
  commentId,
  includeByUserIdAndEmail,
  includeByIP,
  includeByEmailDomain,
  sso
);
[inline-code-end]

---
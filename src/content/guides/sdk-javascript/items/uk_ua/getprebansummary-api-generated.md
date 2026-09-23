---
## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| includeByUserIdAndEmail | boolean | Ні |  |
| includeByIP | boolean | Ні |  |
| includeByEmailDomain | boolean | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`PreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PreBanSummary.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getPreBanSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchSummary(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const includeByUserIdAndEmail: boolean = true;
  const includeByIP: boolean = false;
  const includeByEmailDomain: boolean = true;
  const sso: string = "sso_token_abc";

  const summary: PreBanSummary = await getPreBanSummary(
    tenantId,
    commentId,
    includeByUserIdAndEmail,
    includeByIP,
    includeByEmailDomain,
    sso
  );

  console.log(summary);
}

fetchSummary();
[inline-code-end]

---
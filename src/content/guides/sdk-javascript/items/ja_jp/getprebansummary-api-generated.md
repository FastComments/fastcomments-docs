## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| includeByUserIdAndEmail | boolean | いいえ |  |
| includeByIP | boolean | いいえ |  |
| includeByEmailDomain | boolean | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`PreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PreBanSummary.ts)

## 例

[inline-code-attrs-start title = 'getPreBanSummary の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
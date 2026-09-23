---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| includeByUserIdAndEmail | boolean | 아니오 |  |
| includeByIP | boolean | 아니오 |  |
| includeByEmailDomain | boolean | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`PreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PreBanSummary.ts)

## 예시

[inline-code-attrs-start title = 'getPreBanSummary 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
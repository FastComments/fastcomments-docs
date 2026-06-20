## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| includeByUserIdAndEmail | boolean | לא |  |
| includeByIP | boolean | לא |  |
| includeByEmailDomain | boolean | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`PreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PreBanSummary.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPreBanSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
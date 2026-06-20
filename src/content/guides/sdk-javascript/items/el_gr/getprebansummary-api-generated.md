## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| commentId | string | Ναι |  |
| includeByUserIdAndEmail | boolean | Όχι |  |
| includeByIP | boolean | Όχι |  |
| includeByEmailDomain | boolean | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`PreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PreBanSummary.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPreBanSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
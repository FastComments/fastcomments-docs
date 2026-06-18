## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| errorId | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-72f3b4';
const templateId: string = 'email_template-9c3a1';
let providedErrorId: string | undefined = undefined; // προαιρετική τιμή, μπορεί να οριστεί αλλού
const errorId: string = providedErrorId ?? 'render_err-5d2f7';
const result: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
[inline-code-end]
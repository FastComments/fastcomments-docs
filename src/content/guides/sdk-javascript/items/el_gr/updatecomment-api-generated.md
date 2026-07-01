## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| updatableCommentParams | UpdatableCommentParams | Ναι |  |
| contextUserId | string | Όχι |  |
| doSpamCheck | boolean | Όχι |  |
| isLive | boolean | Όχι |  |

## Απάντηση

Επιστρέφει: [`UpdateCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateCommentResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const updatableCommentParams: UpdatableCommentParams = {
  // παραδειγματικά πεδία· το πραγματικό σχήμα εξαρτάται από τον ορισμό του API
  // π.χ., body: "Edited comment content",
};

const contextUserId: string = "user_abcde";
const doSpamCheck: boolean = true;
const isLive: boolean = false;

const result: UpdateCommentResponse = await updateComment(
  tenantId,
  commentId,
  updatableCommentParams,
  contextUserId,
  doSpamCheck,
  isLive
);
[inline-code-end]

---
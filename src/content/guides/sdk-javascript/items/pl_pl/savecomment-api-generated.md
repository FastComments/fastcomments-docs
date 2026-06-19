## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createCommentParams | CreateCommentParams | Tak |  |
| isLive | boolean | Nie |  |
| doSpamCheck | boolean | Nie |  |
| sendEmails | boolean | Nie |  |
| populateNotifications | boolean | Nie |  |

## Odpowiedź

Zwraca: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APISaveCommentResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład saveComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_98765";
const createCommentParams: CreateCommentParams = {
  content: "This fixed my build pipeline — switching the environment variable resolved the missing dependency.",
  threadId: "blog-post-2026-06-19-ci-troubleshooting",
  userSession: { userId: "user_7542", displayName: "Aisha Khan", email: "aisha.k@example.com" }
};
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = false;
const populateNotifications: boolean = true;
const response: APISaveCommentResponse = await saveComment(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]

---
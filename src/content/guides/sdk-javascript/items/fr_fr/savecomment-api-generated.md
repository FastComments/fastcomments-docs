## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createCommentParams | CreateCommentParams | Oui |  |
| isLive | boolean | Non |  |
| doSpamCheck | boolean | Non |  |
| sendEmails | boolean | Non |  |
| populateNotifications | boolean | Non |  |

## Réponse

Renvoie : [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APISaveCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de saveComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
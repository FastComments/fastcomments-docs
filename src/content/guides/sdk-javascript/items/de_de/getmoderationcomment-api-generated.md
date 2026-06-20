## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| includeEmail | boolean | Nein |  |
| includeIP | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getModerationComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_62b8f9a3e1d4';
const includeEmail: boolean = true;
const includeIP: boolean = false;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4In0.signature';
const response: ModerationAPICommentResponse = await getModerationComment(commentId, includeEmail, includeIP, sso);
[inline-code-end]
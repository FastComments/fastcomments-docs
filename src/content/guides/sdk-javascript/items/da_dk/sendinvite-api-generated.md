## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| fromName | string | Ja |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'sendInvite Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-128';
const id: string = 'comment-8421f';
const fromName: string = 'Marcus Lindström';
const note: string | undefined = undefined; // eksempel på valgfri parameter
const response: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName);
[inline-code-end]
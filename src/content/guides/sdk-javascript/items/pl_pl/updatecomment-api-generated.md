## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| updatableCommentParams | UpdatableCommentParams | Tak |  |
| contextUserId | string | Nie |  |
| doSpamCheck | boolean | Nie |  |
| isLive | boolean | Nie |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3f47b2a1";
const id: string = "comment_9a12b3c4";
const updatableCommentParams: UpdatableCommentParams = {
  body: "Thanks for the update — I've adjusted my view accordingly."
};
const contextUserId: string = "user_8721";
const doSpamCheck: boolean = true;
const isLive: boolean = false;
const result: FlagCommentPublic200Response = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]

---
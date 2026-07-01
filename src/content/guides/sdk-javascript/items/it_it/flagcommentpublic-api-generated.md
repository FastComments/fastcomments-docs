## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| isFlagged | boolean | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`FlagCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublicResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio flagCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoFlagComment() {
    const tenantId: string = "acme-tenant-123";
    const commentId: string = "comment-987654";
    const isFlagged: boolean = true;
    const sso: string = "sso-token-abc123";

    const result: FlagCommentPublicResponse = await flagCommentPublic(
        tenantId,
        commentId,
        isFlagged,
        sso
    );

    console.log(result);
}

demoFlagComment();
[inline-code-end]

---
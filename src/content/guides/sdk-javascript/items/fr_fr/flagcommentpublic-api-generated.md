## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| isFlagged | boolean | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`FlagCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublicResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple flagCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
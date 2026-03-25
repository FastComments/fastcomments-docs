## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| broadcastId | string | Tak |  |
| editKey | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f2c9b';
const commentId: string = 'comment-7c3a9f2d';
const broadcastId: string = 'article-2026-03-20';
const editKey: string | undefined = 'ek_pub_abc12345';
const sso: string | undefined = 'sso_eyJhbGciOiJIUzI1Ni';

const result: DeleteCommentPublic200Response = await deleteCommentPublic(
  tenantId,
  commentId,
  broadcastId,
  editKey,
  sso
);
[inline-code-end]

---
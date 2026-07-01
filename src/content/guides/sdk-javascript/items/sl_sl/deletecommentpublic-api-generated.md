## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Odgovor

Vrne: [`DeleteCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentPublicResponse.ts)

## Primer

[inline-code-attrs-start title = 'deleteCommentPublic Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'c6f9a2e3-9b1d-4f8a-b2d2-1a2b3c4d5e6f';
  const commentId: string = '7d9f0b1c-2e3f-4a5b-6c7d-8e9f0a1b2c3d';
  const broadcastId: string = 'live-2023-09-15';
  const editKey: string = 'ed1tK3y1234567890';
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.abc123';

  const result: DeleteCommentPublicResponse = await deleteCommentPublic(
    tenantId,
    commentId,
    broadcastId,
    editKey,
    sso
  );
})();
[inline-code-end]
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| broadcastId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## Przykład

[inline-code-attrs-start title = 'unPinComment Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-20230915-001";
  const broadcastId: string = "broadcast-2023-09-15";
  const ssoToken: string = "sso-abc123";

  const result: ChangeCommentPinStatusResponse = await unPinComment(tenantId, commentId, broadcastId, ssoToken);
  const resultWithoutSso: ChangeCommentPinStatusResponse = await unPinComment(tenantId, commentId, broadcastId);
})();
[inline-code-end]

---
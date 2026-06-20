## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| banUserUndoParams | BanUserUndoParams | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di postBanUserUndo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const banUserUndoParams: BanUserUndoParams = {
  userId: 'u_9f3d2b7c',
  banId: 'b_58a1e4',
  siteId: 'site_204',
  moderatorId: 'mod_12',
  note: 'Ban removed after manual review; user allowed to repost'
};

const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJtb2QtMTIiLCJpYXQiOjE2Mzk0MzIwMDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

const result: APIEmptyResponse = await postBanUserUndo(banUserUndoParams, ssoToken);
[inline-code-end]

---
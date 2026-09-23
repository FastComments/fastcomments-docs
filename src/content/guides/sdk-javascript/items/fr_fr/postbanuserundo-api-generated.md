## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| banUserUndoParams | BanUserUndoParams | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple postBanUserUndo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9876";

const undoParams: BanUserUndoParams = {
  userId: "user_12345",
  // des champs requis supplémentaires peuvent être ajoutés ici
};

const resultWithoutSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams);

const ssoToken: string = "sso_abcdef123456";

const resultWithSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams, ssoToken);
[inline-code-end]

---
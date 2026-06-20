---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| value | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationPageSearchResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getSearchPages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const searchValue: string = "homepage-recent-threads";
const ssoToken: string = "sso_user_7f9b2c3d";
const resultWithBoth: ModerationPageSearchResponse = await getSearchPages(searchValue, ssoToken);
const resultWithValueOnly: ModerationPageSearchResponse = await getSearchPages(searchValue);
const resultWithSSOOnly: ModerationPageSearchResponse = await getSearchPages(undefined, ssoToken);
[inline-code-end]

---
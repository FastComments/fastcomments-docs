## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| urlId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de putCloseThread'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const urlId: string = 'thread-2f7c9b6a';
const closeResultWithoutSSO: APIEmptyResponse = await putCloseThread(urlId);

const urlIdWithSSO: string = 'thread-8a9b3e1c';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiNjc4OSJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const closeResultWithSSO: APIEmptyResponse = await putCloseThread(urlIdWithSSO, ssoToken);
[inline-code-end]

---
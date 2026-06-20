## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| urlId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de putReopenThread'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const reopenResultWithSso: APIEmptyResponse = await putReopenThread("th_3c9b2a7f", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOjEyM30.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c");
const reopenResultNoSso: APIEmptyResponse = await putReopenThread("th_7a4e5c1d");
[inline-code-end]

---
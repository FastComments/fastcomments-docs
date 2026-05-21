## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |

## Odpowiedź

Zwraca: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const idSuffix: string | undefined = undefined;
const tenantId: string = "acme-enterprises";
const id: string = idSuffix ?? "user_98765";
const response: GetUser200Response = await getUser({ tenantId, id });
[inline-code-end]

---
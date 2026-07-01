## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|---------------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Risposta

Restituisce: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageByURLIdAPIResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getPageByURLId'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";

  const response: GetPageByURLIdAPIResponse = await getPageByURLId(tenantId, urlId);
  const page: APIPage | undefined = response.page;
})();
[inline-code-end]

---
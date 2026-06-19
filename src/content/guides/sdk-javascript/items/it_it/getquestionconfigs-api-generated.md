## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async (): Promise<void> => {
  const tenantId: string = 'acme-corp-78';
  const responseWithoutSkip: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId);
  const responseWithSkip: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId, 25);
  console.log(responseWithoutSkip, responseWithSkip);
})();
[inline-code-end]

---
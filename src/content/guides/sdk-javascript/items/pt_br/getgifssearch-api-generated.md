## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| search | string | Sim |  |
| locale | string | Não |  |
| rating | string | Não |  |
| page | number | Não |  |

## Resposta

Retorna: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_9f8b7c";
  const search: string = "funny cats";
  const locale: string = "en-US";
  const rating: string = "pg";
  const page: number = 1;

  const result: GetGifsSearchResponse = await getGifsSearch(
    tenantId,
    search,
    locale,
    rating,
    page
  );

  console.log(result);
}

demo();
[inline-code-end]

---
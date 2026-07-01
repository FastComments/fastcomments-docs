## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Odpowiedź

Zwraca: [`GetV2PageReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getV2PageReacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetPageReacts(): Promise<void> {
    const tenantId: string = "acme-corp-tenant";
    const urlId: string = "article-2024-06-01";

    const reacts: GetV2PageReactsResponse = await getV2PageReacts(tenantId, urlId);

    // przykładowy dostęp do opcjonalnej właściwości
    const customConfig: CustomConfigParameters | undefined = reacts.customConfig;
    console.log(reacts);
}

demoGetPageReacts();
[inline-code-end]
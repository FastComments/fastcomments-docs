## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |

## Svar

Returnerer: [`GetV2PageReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getV2PageReacts Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetPageReacts(): Promise<void> {
    const tenantId: string = "acme-corp-tenant";
    const urlId: string = "article-2024-06-01";

    const reacts: GetV2PageReactsResponse = await getV2PageReacts(tenantId, urlId);

    // eksempel på valgfri egenskadsadgang
    const customConfig: CustomConfigParameters | undefined = reacts.customConfig;
    console.log(reacts);
}

demoGetPageReacts();
[inline-code-end]
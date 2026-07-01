## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |

## Respons

Retourneert: [`GetV2PageReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getV2PageReacts Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetPageReacts(): Promise<void> {
    const tenantId: string = "acme-corp-tenant";
    const urlId: string = "article-2024-06-01";

    const reacts: GetV2PageReactsResponse = await getV2PageReacts(tenantId, urlId);

    // optioneel voorbeeld van eigenschaptoegang
    const customConfig: CustomConfigParameters | undefined = reacts.customConfig;
    console.log(reacts);
}

demoGetPageReacts();
[inline-code-end]
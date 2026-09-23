## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'putReopenThread Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function reopenThreadExample() {
    const tenantId: string = "tenant_12345";
    const urlId: string = "post-9876";
    const sso: string = "user-abc123";

    const responseWithoutSso: APIEmptyResponse = await putReopenThread(tenantId, urlId);
    const responseWithSso: APIEmptyResponse = await putReopenThread(tenantId, urlId, sso);
}

reopenThreadExample();
[inline-code-end]
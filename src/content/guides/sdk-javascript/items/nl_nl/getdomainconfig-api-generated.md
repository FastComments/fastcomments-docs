## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domain | string | Ja |  |

## Respons

Retourneert: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getDomainConfig Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
type DomainModeration = { moderation?: { enabled?: boolean; mode?: string } };

const tenantId: string = "tc_4b6f9d2a9e1f";
const domain: string = "comments.newsdaily.com";
const config: GetDomainConfigResponse = await getDomainConfig(tenantId, domain);

const moderationEnabled: boolean | undefined = (config as unknown as DomainModeration).moderation?.enabled;
const moderationMode: string | undefined = (config as unknown as DomainModeration).moderation?.mode;
[inline-code-end]
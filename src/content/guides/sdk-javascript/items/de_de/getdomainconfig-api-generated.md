## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domain | string | Ja |  |

## Antwort

Rückgabe: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getDomainConfig Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
type DomainModeration = { moderation?: { enabled?: boolean; mode?: string } };

const tenantId: string = "tc_4b6f9d2a9e1f";
const domain: string = "comments.newsdaily.com";
const config: GetDomainConfigResponse = await getDomainConfig(tenantId, domain);

const moderationEnabled: boolean | undefined = (config as unknown as DomainModeration).moderation?.enabled;
const moderationMode: string | undefined = (config as unknown as DomainModeration).moderation?.mode;
[inline-code-end]

---
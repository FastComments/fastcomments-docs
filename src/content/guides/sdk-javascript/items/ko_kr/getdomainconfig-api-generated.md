## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| domain | string | 예 |  |

## 응답

반환값: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigResponse.ts)

## 예제

[inline-code-attrs-start title = 'getDomainConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
type DomainModeration = { moderation?: { enabled?: boolean; mode?: string } };

const tenantId: string = "tc_4b6f9d2a9e1f";
const domain: string = "comments.newsdaily.com";
const config: GetDomainConfigResponse = await getDomainConfig(tenantId, domain);

const moderationEnabled: boolean | undefined = (config as unknown as DomainModeration).moderation?.enabled;
const moderationMode: string | undefined = (config as unknown as DomainModeration).moderation?.mode;
[inline-code-end]

---
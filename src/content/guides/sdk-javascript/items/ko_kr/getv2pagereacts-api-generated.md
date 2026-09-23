## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## 예시

[inline-code-attrs-start title = 'getV2PageReacts 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-42";
const urlId: string = "article-9876";
const ssoToken: string = "sso-abc123";

const reactsWithSso: GetV2PageReacts = await getV2PageReacts(tenantId, urlId, ssoToken);
const reactsWithoutSso: GetV2PageReacts = await getV2PageReacts(tenantId, urlId);
[inline-code-end]
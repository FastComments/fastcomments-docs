## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| urlId | string | 예 |  |
| tenantId | string | 아니오 |  |
| sso | string | 아니오 |  |

## Response

반환: [`PutCloseThreadResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutCloseThreadResponse.ts)

## Example

[inline-code-attrs-start title = 'putCloseThread 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function closeThreadDemo(): Promise<void> {
  const urlId: string = "article-2023-09-15";
  const tenantId: string = "tenant-42";
  const sso: string = "sso-token-xyz";

  const response: PutCloseThreadResponse = await putCloseThread(urlId, tenantId, sso);
  console.log(response);
}

closeThreadDemo();
[inline-code-end]
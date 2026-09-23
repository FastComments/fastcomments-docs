## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예시

[inline-code-attrs-start title = 'putReopenThread 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
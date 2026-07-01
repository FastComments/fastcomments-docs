## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`GetV2PageReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactsResponse.ts)

## 예시

[inline-code-attrs-start title = 'getV2PageReacts 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetPageReacts(): Promise<void> {
    const tenantId: string = "acme-corp-tenant";
    const urlId: string = "article-2024-06-01";

    const reacts: GetV2PageReactsResponse = await getV2PageReacts(tenantId, urlId);

    // 선택적 속성 접근 예시
    const customConfig: CustomConfigParameters | undefined = reacts.customConfig;
    console.log(reacts);
}

demoGetPageReacts();
[inline-code-end]

---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 아니오 |  |
| createHashTagBody | CreateHashTagBody | 아니오 |  |

## 응답

반환: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTag200Response.ts)

## 예제

[inline-code-attrs-start title = 'addHashTag 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string | undefined = "tenant_3c9f7b";
const createHashTagBody: CreateHashTagBody = {
  name: "support",
  title: "Support",
  description: "Questions about product usage, bugs, and account issues",
  color: "#0066CC",
  isActive: true,
  aliases: ["help", "customer-service"]
};
const result: AddHashTag200Response = await addHashTag(tenantId, createHashTagBody);
[inline-code-end]

---
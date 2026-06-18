## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createEmailTemplateBody | CreateEmailTemplateBody | 예 |  |

## 응답

반환: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## 예제

[inline-code-attrs-start title = 'createEmailTemplate 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4f2b1c9e";
const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "New Comment Notification",
  subject: "Someone replied to your discussion",
  fromName: "Community Team",
  fromAddress: "no-reply@community.example.com",
  htmlBody: "<p>\{{comment.author}} replied: \{{comment.text}}</p>",
  plaintextBody: "\{{comment.author}} replied: \{{comment.text}}",
  previewText: "A new reply on a discussion you follow",
  isDefault: false // 선택적 매개변수 사용을 보여주는 플래그
};
const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]

---
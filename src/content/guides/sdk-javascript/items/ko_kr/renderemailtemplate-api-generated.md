## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | 예 |  |
| locale | string | 아니요 |  |

## 응답

반환: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplate200Response.ts)

## 예제

[inline-code-attrs-start title = 'renderEmailTemplate 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_b6f3c2';
const renderEmailTemplateBody: RenderEmailTemplateBody = {
  templateId: 'comment-notification',
  recipient: { name: 'Ava Thompson', email: 'ava.thompson@publisher.com' },
  context: {
    siteName: 'City Gazette',
    commentText: 'Thanks for the in-depth coverage — very helpful.',
    articleTitle: 'Downtown Redevelopment Plan Advances',
    threadUrl: 'https://citygazette.example/articles/2026/redevelopment#comments'
  }
};
const locale: string = 'en-US';
const result: RenderEmailTemplate200Response = await renderEmailTemplate(tenantId, renderEmailTemplateBody, locale);
[inline-code-end]

---
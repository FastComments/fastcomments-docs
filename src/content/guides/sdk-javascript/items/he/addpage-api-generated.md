## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIPageData | CreateAPIPageData | Yes |  |

## תגובה

מחזירה: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddPageAPIResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת addPage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8432b1-4c2e-4f9d-8a5d-7bc9e2f6d9a0";

const pageData: CreateAPIPageData = {
    url: "https://example.com/articles/fastcomments-intro",
    title: "FastComments Introduction",
    description: "An introductory guide to FastComments API usage.",
    tags: ["fastcomments", "api", "tutorial"],
    metadata: {
        author: "Jane Doe",
        publishedAt: "2024-03-15T08:00:00Z"
    }
};

const response: AddPageAPIResponse = await addPage(tenantId, pageData);
[inline-code-end]
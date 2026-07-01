## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPIPageData | UpdateAPIPageData | Yes |  |

## תגובה

מחזיר: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchPageAPIResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'patchPage דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const pageId: string = "page_98765";

  const updateData: UpdateAPIPageData = {
    title: "Updated FAQ Page"
    // description?: string ניתן להשמיט
  };

  const response: PatchPageAPIResponse = await patchPage(tenantId, pageId, updateData);
})();
[inline-code-end]

---
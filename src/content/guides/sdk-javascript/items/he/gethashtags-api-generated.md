## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| page | number | לא |  |

## תגובה

Returns: [`GetHashTagsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse1.ts)

## דוגמה

[inline-code-attrs-start title = 'getHashTags דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";

  const responseWithPage: GetHashTagsResponse1 = await getHashTags(tenantId, 1);
  const responseDefault: GetHashTagsResponse1 = await getHashTags(tenantId);

  console.log(responseWithPage, responseDefault);
})();
[inline-code-end]
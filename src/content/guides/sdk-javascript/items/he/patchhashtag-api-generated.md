## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| tag | string | כן |  |
| updateHashTagBody | UpdateHashTagBody | לא |  |

## תגובה

מחזיר: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'patchHashTag דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // קריאה ללא גוף אופציונלי
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // הכנת גוף לעדכון
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]
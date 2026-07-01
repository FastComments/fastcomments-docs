## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tag | string | כן |  |
| tenantId | string | לא |  |
| updateHashTagBody | UpdateHashTagBody | לא |  |

## Response

מחזיר: [`PatchHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTagResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'patchHashTag דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const response1: PatchHashTagResponse = await patchHashTag("new-feature");

const response2: PatchHashTagResponse = await patchHashTag(
  "beta-release",
  "tenant-9f8b7c"
);

const updateBody: UpdateHashTagBody = {
  description: "Mark comments related to the upcoming beta release",
  color: "#1e90ff"
};

const response3: PatchHashTagResponse = await patchHashTag(
  "beta-release",
  "tenant-9f8b7c",
  updateBody
);
[inline-code-end]
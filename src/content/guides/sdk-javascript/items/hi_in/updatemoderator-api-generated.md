## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updateModeratorBody | UpdateModeratorBody | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'updateModerator उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const moderatorId: string = "mod_12345";

const updateBody: UpdateModeratorBody = {
  isActive: true,
  // role?: string वैकल्पिक है और यहाँ छोड़ा गया है
};

const result: APIEmptyResponse = await updateModerator(tenantId, moderatorId, updateBody);
[inline-code-end]
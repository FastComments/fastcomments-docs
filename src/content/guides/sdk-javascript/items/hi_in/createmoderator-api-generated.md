## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createModeratorBody | CreateModeratorBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createModerator उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const moderatorPayload: CreateModeratorBody = {
  userId: "user_9876",
  // वैकल्पिक फ़ील्ड; आवश्यकता न होने पर छोड़ा जा सकता है
  notes: "Temporary moderator for event"
};

const response: CreateModeratorResponse = await createModerator(tenantId, moderatorPayload);
[inline-code-end]
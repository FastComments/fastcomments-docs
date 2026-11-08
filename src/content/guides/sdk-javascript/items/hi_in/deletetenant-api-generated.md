## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| sure | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'deleteTenant उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_9876";

  const resultWithoutSure: APIEmptyResponse = await deleteTenant(tenantId, userId);
  const resultWithSure: APIEmptyResponse = await deleteTenant(tenantId, userId, "confirm");
}

main();
[inline-code-end]
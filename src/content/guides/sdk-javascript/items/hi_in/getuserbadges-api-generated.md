## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| userId | string | नहीं |  |
| badgeId | string | नहीं |  |
| type | number | नहीं |  |
| displayedOnComments | boolean | नहीं |  |
| limit | number | नहीं |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgesResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getUserBadges उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";

  // सभी पैरामीटर दिए गए हैं
  const fullResponse: APIGetUserBadgesResponse = await getUserBadges(
    tenantId,
    "user-42",
    "badge-premium",
    1,
    true,
    10,
    0
  );

  // केवल आवश्यक और एक वैकल्पिक पैरामीटर (limit)
  const limitedResponse: APIGetUserBadgesResponse = await getUserBadges(
    tenantId,
    undefined,
    undefined,
    undefined,
    undefined,
    5
  );

  console.log(fullResponse, limitedResponse);
})();
[inline-code-end]
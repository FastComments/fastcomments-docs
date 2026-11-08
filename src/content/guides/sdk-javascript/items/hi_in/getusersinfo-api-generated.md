किसी टेनेंट के लिए बल्क यूज़र जानकारी। दिए गए userIds के लिए, User / SSOUser से डिस्प्ले जानकारी लौटाता है।
कमेंट विजेट द्वारा उन यूज़र्स को समृद्ध करने के लिए उपयोग किया जाता है जो अभी presence event के माध्यम से दिखाई दिए।
कोई पेज संदर्भ नहीं: गोपनीयता समान रूप से लागू की जाती है (प्राइवेट प्रोफाइल मास्क कर दिए जाते हैं)।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| ids | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getUsersInfo उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]
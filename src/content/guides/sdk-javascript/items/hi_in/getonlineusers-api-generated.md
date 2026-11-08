किसी पेज के वर्तमान में ऑनलाइन दर्शक: वे लोग जिनका websocket सत्र इस समय पेज के लिए सब्सक्राइब है।
anonCount + totalCount लौटाता है (पूरे रूम के सब्सक्राइबर, जिनमें वे अनाम दर्शक भी शामिल हैं जिन्हें हम सूचीबद्ध नहीं करते)।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| afterName | string | नहीं |  |
| afterUserId | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getOnlineUsers उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "article-9876";
  const afterName: string | undefined = "john_doe";
  const afterUserId: string | undefined = "user_456";

  const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(onlineUsers);
}

fetchOnlineUsers();
[inline-code-end]
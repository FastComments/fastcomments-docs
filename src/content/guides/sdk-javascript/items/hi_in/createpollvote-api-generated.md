पोल पर वोट दर्ज करें, या किसी मौजूदा वोट को दूसरे विकल्प पर स्थानांतरित करें। एक मतदाता का प्रति पोल अधिकतम एक वोट होता है, इसलिए उसी मतदाता के लिए इसे दोबारा कॉल करने से नया वोट जोड़ने के बजाय उनका वोट स्थानांतरित हो जाता है।

यह साइट की पोल सेटिंग्स का पालन करता है: यदि वोटिंग केवल लॉग-इन उपयोगकर्ताओं के लिए सेट है, तो केवल anonUserId वाला वोट अस्वीकार कर दिया जाता है, और गुमनाम वोट प्रति IP प्रति पोल दर सीमित (rate limited) होते हैं।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createPollVoteBody | CreatePollVoteBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createPollVote उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId वैकल्पिक है और यहाँ छोड़ा गया है
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]
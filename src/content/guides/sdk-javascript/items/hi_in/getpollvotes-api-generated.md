एक पोल की गिनती के पीछे के व्यक्तिगत वोट, सबसे पुराने पहले।

एक पोल एक कमेंट से संबंधित होता है, इसलिए वोट हमेशा एक बार में एक पोल के लिए पढ़े जाते हैं - commentId आवश्यक है। इससे हर क्वेरी संग्रह (collection) में पहले से मौजूद इंडेक्स का उपयोग करती है।

यह पोल की गोपनीयता का पालन करता है: एक गुमनाम पोल के वोट नहीं पढ़े जा सकते (poll-anonymous), यहाँ या id द्वारा।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| voterId | string | नहीं |  |
| optionId | string | नहीं |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getPollVotes उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]
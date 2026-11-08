## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getVotes उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-67890";

(async () => {
  const votesResponse: GetVotesResponse = await getVotes(tenantId, urlId);
  // प्रतिक्रिया से एक वैकल्पिक प्रॉपर्टी तक पहुँचने का उदाहरण
  const firstVote: PublicVote | undefined = votesResponse.votes?.[0];
})();
[inline-code-end]
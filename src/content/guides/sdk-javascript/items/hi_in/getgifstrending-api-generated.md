## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| locale | string | नहीं |  |
| rating | string | नहीं |  |
| page | number | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrendingResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getGifsTrending उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6d";
  const locale: string = "en-US";
  const rating: string = "PG-13";
  const page: number = 1;

  const trendingAll: GetGifsTrendingResponse = await getGifsTrending(tenantId, locale, rating, page);
  console.log(trendingAll);

  // केवल आवश्यक पैरामीटर का उपयोग करते हुए
  const trendingMinimal: GetGifsTrendingResponse = await getGifsTrending(tenantId);
  console.log(trendingMinimal);
}

runExample();
[inline-code-end]
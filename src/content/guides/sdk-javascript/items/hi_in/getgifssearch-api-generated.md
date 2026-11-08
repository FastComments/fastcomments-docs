## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| search | string | हाँ |  |
| locale | string | नहीं |  |
| rating | string | नहीं |  |
| page | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getGifsSearch उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const search: string = "celebration fireworks";
  const locale: string = "fr-FR";
  const rating: string = "g";
  const page: number = 1;

  const fullResult: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
  const minimalResult: GetGifsSearchResponse = await getGifsSearch(tenantId, "dog memes");
}

runExample();
[inline-code-end]
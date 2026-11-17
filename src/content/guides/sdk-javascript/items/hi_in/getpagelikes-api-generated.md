किसी पेज के लाइक की संख्या, वैकल्पिक रूप से उन उपयोगकर्ताओं के साथ जिन्होंने लाइक किया।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| includeUserInfo | PageReactIncludeUserInfo | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageLikesAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getPageLikes उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';
const urlId: string = 'https://mysite.com/blog/best-coffee-2024';

// आवश्यक पैरामीटर के साथ बेसिक कॉल
const likesResponse: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId);
console.log(`Page has ${likesResponse.pageLikes.length} likes`);

// हर लाइक के लिए उपयोगकर्ता जानकारी शामिल करें
const likesWithUsers: GetPageLikesAPIResponse = await getPageLikes(tenantId, urlId, 'true');
likesWithUsers.pageLikes.forEach((like: PageReactUser) => {
  console.log(`${like.name} liked this page on ${like.date}`);
});
[inline-code-end]
दिए गए कमेंट्स को विजेट लोकेल में अनुवादित करता है। केवल तब उपलब्ध जब संगत विजेट
कस्टमाइज़ेशन रूल में Comment Translation को Automatic या Manual पर सेट किया गया हो।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| commentTranslationsRequest | CommentTranslationsRequest | हाँ |  |
| sso | string | नहीं |  |
| locale | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'translateComments उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'blog-post-42';
const sso: string = 'JWT_TOKEN_HERE';

const commentTranslationsRequest: CommentTranslationsRequest = {
  commentIds: ['c_98765', 'c_54321'],
  targetLocale: 'es'
};

const locale: string = 'es-ES';

const response: CommentTranslationsResponse = await translateComments(
  tenantId,
  urlId,
  commentTranslationsRequest,
  sso,
  locale
);

console.log(`Translated ${response.translations.length} comments`);
[inline-code-end]
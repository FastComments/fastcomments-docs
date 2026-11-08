## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| namespace | string | हाँ |  |
| component | string | हाँ |  |
| locale | string | नहीं |  |
| useFullTranslationIds | boolean | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getTranslations उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTranslations() {
  const adminDashboard: GetTranslationsResponse = await getTranslations('admin', 'dashboard');
  const userProfileFr: GetTranslationsResponse = await getTranslations('user', 'profile', 'fr-FR', true);
}
[inline-code-end]
[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

वर्तमान उपयोगकर्ता की एक प्रतिक्रिया को पेज से हटाता है। यदि उपयोगकर्ता ने वह प्रतिक्रिया नहीं जोड़ी है, तो अनुरोध `no-react` कोड के साथ सफल होता है और काउंट को नहीं बदलता।

[inline-code-attrs-start title = 'पेज रिएक्ट डिलीट cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'पेज रिएक्ट डिलीट अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'पेज रिएक्ट डिलीट प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' when the user had not added this reaction. Otherwise included on failure. **/
    code?: 'no-react' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
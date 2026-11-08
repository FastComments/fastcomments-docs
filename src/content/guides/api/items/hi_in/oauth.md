FastComments एक OAuth 2.1 ऑथराइज़ेशन सर्वर है। एक एप्लिकेशन एक टोकन प्राप्त कर सकता है जो एक FastComments खाते से बंधा होता है और इस गाइड में हर एंडपॉइंट पर API की के स्थान पर उसका उपयोग कर सकता है। Zapier ऐप, MCP सर्वर, और अन्य थर्ड-पार्टी इंटीग्रेशन इसी तरह कनेक्ट होते हैं।

टोकन PKCE के साथ ऑथराइज़ेशन कोड फ्लो के माध्यम से जारी किए जाते हैं। कोई client credentials या implicit grant नहीं है।

### डिस्कवरी

एंडपॉइंट के स्थान, समर्थित ग्रांट, और ऑथ मेथड्स मानक मेटाडेटा URL पर प्रकाशित होते हैं:

[inline-code-attrs-start title = 'ऑथराइज़ेशन सर्वर मेटाडेटा'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

इसके द्वारा वर्णित एंडपॉइंट:

[inline-code-attrs-start title = 'OAuth एंडपॉइंट'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

EU क्षेत्र के खाते `https://eu.fastcomments.com` को issuer के रूप में उपयोग करते हैं, उसी पाथ के साथ।

### क्लाइंट रजिस्टर करना

फ्लो शुरू करने से पहले एक क्लाइंट को `client_id` और एक रजिस्टर्ड `redirect_uri` की आवश्यकता होती है। इसे प्राप्त करने के दो तरीके हैं:

- **डायनामिक क्लाइंट रजिस्ट्रेशन।** RFC 7591 के अनुसार JSON बॉडी के साथ `POST /oauth/register` (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`)। रिस्पॉन्स में `client_id` और, कॉन्फिडेंशियल क्लाइंट्स के लिए, `client_secret` होता है। रजिस्ट्रेशन बिना ऑथेंटिकेशन के होता है और प्रति IP रेट लिमिटेड है।
- **क्लाइंट ID मेटाडेटा डॉक्यूमेंट।** क्लाइंट अपने नियंत्रण वाले एक `https` URL को अपने `client_id` के रूप में उपयोग करता है। FastComments उस URL को फ़ेच करता है और उससे वही मेटाडेटा फ़ील्ड पढ़ता है। किसी रजिस्ट्रेशन कॉल की आवश्यकता नहीं है।

FastComments डैशबोर्ड में सूचीबद्ध पार्टनर एप्लिकेशन, जैसे Zapier, सीधे FastComments द्वारा रजिस्टर किए जाते हैं। यदि आप मार्केटप्लेस लिस्टिंग बना रहे हैं और फर्स्ट-पार्टी क्लाइंट की आवश्यकता है तो सपोर्ट से संपर्क करें।

### स्कोप

[inline-code-attrs-start title = 'स्कोप'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

कोई स्कोप न मांगने वाला रिक्वेस्ट दोनों प्राप्त करता है। उपयोगकर्ता को कंसेंट पेज पर अनुरोधित स्कोप दिखाई देते हैं। इन दो के अलावा किसी अन्य स्कोप का अनुरोध `invalid_scope` के साथ विफल हो जाता है।

### चरण 1 - ऑथराइज़ेशन रिक्वेस्ट

उपयोगकर्ता के ब्राउज़र को authorize एंडपॉइंट पर भेजें। हर क्लाइंट के लिए `S256` मेथड के साथ PKCE आवश्यक है।

[inline-code-attrs-start title = 'ऑथराइज़ेशन रिक्वेस्ट'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = 'ऑथराइज़ेशन रिक्वेस्ट पैरामीटर'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** क्लाइंट के रजिस्टर्ड redirect URI में से किसी एक से बिल्कुल मेल खाना चाहिए। **/
    redirect_uri: string
    /** स्पेस से अलग किया गया। दोनों स्कोप मांगने के लिए छोड़ दें। **/
    scope?: 'read' | 'write' | 'read write'
    /** रीडायरेक्ट पर अपरिवर्तित लौटाया जाता है। कॉलबैक को उस सेशन से बांधने के लिए उपयोग करें जिसने फ्लो शुरू किया। **/
    state?: string
    /** base64url(sha256(code_verifier))। **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** वैकल्पिक RFC 8707 रिसोर्स इंडिकेटर। यदि भेजा गया, तो वही मान token एंडपॉइंट को भेजा जाना चाहिए। **/
    resource?: string
}
[inline-code-end]

उपयोगकर्ता आवश्यकता होने पर FastComments में साइन इन करता है और एक कंसेंट पेज देखता है जिसमें आपका एप्लिकेशन, वह खाता जिससे इसे कनेक्ट किया जाएगा, और अनुरोधित स्कोप नामित होते हैं। उपयोगकर्ता के पास उस खाते पर **API Admin** अनुमति होनी चाहिए; कोई अन्य व्यक्ति कंसेंट फॉर्म के बजाय अनुमति त्रुटि देखता है। स्वीकृति करने पर ब्राउज़र आपके `redirect_uri` पर `code` और `state` के साथ रीडायरेक्ट होता है। अस्वीकृति करने पर `error=access_denied` के साथ रीडायरेक्ट होता है।

ऑथराइज़ेशन कोड 10 मिनट के लिए मान्य है और इसे एक बार एक्सचेंज किया जा सकता है। उसी कोड का दूसरा एक्सचेंज पहले एक्सचेंज द्वारा उत्पादित हर टोकन को रद्द कर देता है।

### चरण 2 - टोकन रिक्वेस्ट

कोड को टोकन के लिए एक्सचेंज करें। बॉडी फॉर्म एनकोडेड है। कॉन्फिडेंशियल क्लाइंट्स `client_secret_basic` (HTTP Basic) या `client_secret_post` (बॉडी में सीक्रेट) के साथ ऑथेंटिकेट करते हैं। पब्लिक क्लाइंट्स केवल `client_id` भेजते हैं।

[inline-code-attrs-start title = 'टोकन रिक्वेस्ट cURL उदाहरण'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = 'टोकन रिक्वेस्ट बॉडी (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** केवल कॉन्फिडेंशियल क्लाइंट्स। इसके बजाय HTTP Basic auth के रूप में भेजा जा सकता है। **/
    client_secret?: string
    code: string
    code_verifier: string
    /** भेजे जाने पर ऑथराइज़ेशन रिक्वेस्ट से मेल खाना चाहिए। **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'टोकन रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** fcat_ प्रीफिक्स। एक घंटे के लिए मान्य। **/
    access_token: string
    token_type: 'bearer'
    /** एक्सेस टोकन के समाप्त होने तक सेकंड। 3600। **/
    expires_in: number
    /** fcrt_ प्रीफिक्स। जारी होने से 30 दिनों तक मान्य। **/
    refresh_token: string
    /** स्पेस से अलग किए गए प्रदान किए गए स्कोप। **/
    scope: string
}
[inline-code-end]

त्रुटियां RFC 6749 का पालन करती हैं: `error` और `error_description` के साथ JSON बॉडी, `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` और `unsupported_grant_type` के लिए HTTP 400, `invalid_client` के लिए HTTP 401, रेट लिमिटेड होने पर HTTP 429।

### चरण 3 - API कॉल करना

एक्सेस टोकन को बेयरर टोकन के रूप में भेजें। टेनेंट टोकन द्वारा निहित होता है, इसलिए `tenantId` वैकल्पिक है। दिया जाने पर इसे टोकन से मेल खाना चाहिए अन्यथा रिक्वेस्ट विफल हो जाती है।

[inline-code-attrs-start title = 'बेयरर टोकन cURL उदाहरण'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` टेनेंट, ऑथराइज़िंग उपयोगकर्ता, और प्रदान किए गए स्कोप लौटाता है, जो इसे कनेक्शन टेस्ट के लिए सही कॉल बनाता है। समाप्त या रद्द किए गए टोकन वाली रिक्वेस्ट को HTTP 401 मिलता है। जिस मेथड को टोकन के पास नहीं है उस स्कोप की आवश्यकता वाली रिक्वेस्ट को HTTP 403 मिलता है।

### चरण 4 - रिफ्रेश करना

[inline-code-attrs-start title = 'रिफ्रेश रिक्वेस्ट cURL उदाहरण'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'टोकन रिक्वेस्ट बॉडी (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** वैकल्पिक। मूल रूप से प्रदान किए गए स्कोप के एक सबसेट तक सीमित करता है। **/
    scope?: string
    resource?: string
}
[inline-code-end]

रिस्पॉन्स की आकृति कोड एक्सचेंज के समान है। रिफ्रेश टोकन रोटेट होते हैं: प्रत्येक रिफ्रेश एक नया `refresh_token` लौटाता है और समवर्ती रिक्वेस्ट्स के लिए 30 सेकंड की ग्रेस विंडो के बाद पुराने को रद्द कर देता है। 30 सेकंड से अधिक पहले रोटेट किए गए रिफ्रेश टोकन को प्रस्तुत करना रिप्ले माना जाता है और पूरे ग्रांट को रद्द कर देता है। FastComments द्वारा रजिस्टर किए गए पार्टनर एप्लिकेशन रोटेशन से मुक्त हैं और उन्हें वही रिफ्रेश टोकन वापस मिलता है जिसकी समाप्ति अन्य 30 दिनों तक बढ़ा दी जाती है।

एक रिफ्रेश यह भी पुनः जांचता है कि ऑथराइज़िंग उपयोगकर्ता अभी भी खाते पर API Admin है। यदि नहीं, तो ग्रांट रद्द कर दिया जाता है और रिस्पॉन्स `invalid_grant` होता है।

### रद्दीकरण

[inline-code-attrs-start title = 'रिवोक रिक्वेस्ट cURL उदाहरण'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

एक रिफ्रेश टोकन को रद्द करने से उसी ग्रांट से जारी हर एक्सेस टोकन रद्द हो जाता है। एक एक्सेस टोकन को रद्द करने से केवल वही टोकन रद्द होता है। एंडपॉइंट RFC 7009 के अनुसार, टोकन मिले या न मिले, खाली JSON ऑब्जेक्ट के साथ HTTP 200 लौटाता है।

उपयोगकर्ता FastComments डैशबोर्ड में **Connected Apps** से भी कनेक्शन रद्द कर सकते हैं। उस एप्लिकेशन का हर टोकन तुरंत काम करना बंद कर देता है।
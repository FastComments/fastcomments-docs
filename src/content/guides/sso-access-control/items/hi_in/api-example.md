यहाँ हम FastComments API को कॉल करके एक्सेस कंट्रोल सेटअप करने की प्रक्रिया देखेंगे।

शुरू करने से पहले, ध्यान दें कि हमें स्पष्ट रूप से एक `Group` संरचना बनाने की आवश्यकता नहीं है। ग्रुप केवल पहचानकर्ता (identifiers) होते हैं जो `Users` और `Pages` में जोड़े जाते हैं। किसी यूज़र या पेज में ग्रुप जोड़ने से वह ग्रुप स्वतः "बन" जाता है।

सबसे पहले, आइए दो यूज़र बनाते हैं, `User A` और `User B`, हम उन्हें `Group X` में शुरू करेंगे:

[inline-code-attrs-start title = 'User A बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'User B बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

अब आइए एक `Page` बनाते हैं। हम इसे अपना `Confidential Page` कहेंगे, और अभी तक इनमें से किसी भी यूज़र को इसका एक्सेस नहीं होगा क्योंकि यह `CONFIDENTIAL` ग्रुप में होगा:

[inline-code-attrs-start title = 'Page POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

यूज़र A और B को इस समय नए पेज का एक्सेस **नहीं** है। हालाँकि, चूँकि वे एक ही ग्रुप, `GROUP-X`, में हैं, वे एक-दूसरे को `@mention` कर सकते हैं।

आइए `User B` को अपडेट करें ताकि वह अब पेज का एक्सेस कर सके:

[inline-code-attrs-start title = 'User B अपडेट करने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

अब `User B` दोनों ग्रुप का सदस्य है। हमारे यूज़र अभी भी एक-दूसरे को `@mention` कर सकते हैं, लेकिन केवल `User B` ही हमारा कॉन्फिडेंशियल पेज देख सकता है।

आइए ऐसा बनाते हैं कि `User B` केवल कॉन्फिडेंशियल पेज देख सके:

[inline-code-attrs-start title = 'User B अपडेट करने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

अब वह कॉन्फिडेंशियल पेज देख सकता है, लेकिन हमारे दोनों यूज़र एक-दूसरे को `@mention` नहीं कर सकते, क्योंकि वे अलग-अलग ग्रुप में हैं।

हालाँकि, कोई भी यूज़र जो एक्सेस कंट्रोल का हिस्सा नहीं है, **वह हमारे पेज का एक्सेस कर सकेगा**। इसे रोकने के लिए, सुनिश्चित करें कि किसी SSO यूज़र का `groupIds` null पर सेट न हो। उदाहरण के लिए, आइए `User C` बनाते हैं, जिसे सब कुछ एक्सेस करने की अनुमति है:

[inline-code-attrs-start title = 'User C बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

`groupIds` को null पर सेट करके, हम कहते हैं कि वे एक्सेस कंट्रोल द्वारा सीमित नहीं हैं।

अब, आइए एक ऐसा पेज बनाते हैं जिसका एक्सेस सभी को हो:

[inline-code-attrs-start title = 'Page POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

`accessibleByGroupIds` को null पर सेट करके, हम कहते हैं कि यह `Page` एक्सेस कंट्रोल द्वारा नियंत्रित नहीं है, और दोनों यूज़र इसका एक्सेस कर सकते हैं।

इसके साथ हमारी एक्सेस कंट्रोल के लिए API वॉक-थ्रू पूरी हुई।
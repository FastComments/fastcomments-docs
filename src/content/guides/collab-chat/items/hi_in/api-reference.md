### API अवलोकन

Collab Chat प्रोग्रामेटिक रूप से टेक्स्ट बातचीत प्रबंधित करने के लिए तीन REST API एंडपॉइंट प्रदान करता है। ये एंडपॉइंट आपको ब्राउज़र विजेट का उपयोग किए बिना एनोटेशन प्राप्त करने, बनाने और हटाने की अनुमति देते हैं।

ये सार्वजनिक एंडपॉइंट हैं जो ब्राउज़र कुकीज़ के माध्यम से उपयोगकर्ताओं को प्रमाणित करते हैं। ये API कुंजियों का उपयोग नहीं करते। इन एंडपॉइंट तक पहुँचने के लिए उपयोगकर्ताओं का अपने ब्राउज़र में FastComments में लॉग इन होना आवश्यक है।

### बेस URL

सभी Collab Chat API एंडपॉइंट निम्नलिखित के अंतर्गत हैं:

[inline-code-attrs-start title = 'बेस URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### प्रमाणीकरण

ये एंडपॉइंट ब्राउज़र कुकीज़ के माध्यम से उपयोगकर्ताओं को प्रमाणित करते हैं। ये API कुंजियों का उपयोग नहीं करते। इन एंडपॉइंट तक पहुँचने के लिए उपयोगकर्ताओं का अपने ब्राउज़र में FastComments में लॉग इन होना आवश्यक है।

### सभी बातचीत प्राप्त करें

किसी विशिष्ट पेज के लिए सभी टेक्स्ट बातचीत प्राप्त करें।

#### एंडपॉइंट

[inline-code-attrs-start title = 'GET एंडपॉइंट'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### पैरामीटर

`tenantId` (पाथ पैरामीटर, आवश्यक) आपकी FastComments Tenant ID है।

`urlId` (क्वेरी पैरामीटर, आवश्यक) वह पेज पहचानकर्ता है जिसके लिए आप बातचीत प्राप्त करना चाहते हैं।

#### प्रतिक्रिया

प्रतिक्रिया में API स्थिति, प्रमाणित होने पर वर्तमान उपयोगकर्ता सत्र की जानकारी, उनकी IDs, URLs और टेक्स्ट रेंज के साथ बातचीत की एक सरणी, एक साफ़ किया गया URL पहचानकर्ता, यह दर्शाने वाला फ़्लैग कि वर्तमान उपयोगकर्ता साइट एडमिन या मॉडरेटर है, और लाइव सिंक के लिए WebSocket कनेक्शन विवरण जिसमें `tenantIdWS`, `urlIdWS` और `userIdWS` शामिल हैं।

#### उदाहरण अनुरोध

[inline-code-attrs-start title = 'GET अनुरोध उदाहरण'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### उदाहरण प्रतिक्रिया

[inline-code-attrs-start title = 'GET प्रतिक्रिया उदाहरण'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### बातचीत बनाएँ

किसी विशिष्ट टेक्स्ट चयन के लिए एक नई टेक्स्ट बातचीत बनाएँ।

#### एंडपॉइंट

[inline-code-attrs-start title = 'POST एंडपॉइंट'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### पैरामीटर

`tenantId` (पाथ पैरामीटर, आवश्यक) आपकी FastComments Tenant ID है।

अनुरोध बॉडी JSON होनी चाहिए और इसमें ये आवश्यक फ़ील्ड शामिल होने चाहिए।

`urlId` (स्ट्रिंग, आवश्यक) बेस पेज पहचानकर्ता है।

`urlIdWithRange` (स्ट्रिंग, आवश्यक) टेक्स्ट रेंज के साथ संयुक्त URL है, उदाहरण के लिए `my-page:p:0:15,0:45{abc123}`।

`pageTitle` (स्ट्रिंग, आवश्यक) पेज का शीर्षक है।

`selector` (स्ट्रिंग, आवश्यक) चयनित टेक्स्ट वाले तत्व का DOM पाथ है।

`range` (स्ट्रिंग, आवश्यक) `startOffset:endOffset,startOffset:endOffset{checksum}` प्रारूप में क्रमबद्ध टेक्स्ट रेंज है।

`createdFromCommentId` (स्ट्रिंग, आवश्यक) उस टिप्पणी की ID है जिसने यह चैट शुरू की।

`broadcastId` (स्ट्रिंग, आवश्यक) इको प्रभावों को रोकने के लिए लाइव सिंक्रोनाइज़ेशन के लिए एक UUID है।

#### प्रतिक्रिया

प्रतिक्रिया में API स्थिति और नई बनाई गई बातचीत की ID शामिल है।

#### उदाहरण अनुरोध

[inline-code-attrs-start title = 'POST अनुरोध उदाहरण'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### उदाहरण प्रतिक्रिया

[inline-code-attrs-start title = 'POST प्रतिक्रिया उदाहरण'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### बातचीत हटाएँ

एक मौजूदा टेक्स्ट बातचीत हटाएँ। इस एंडपॉइंट के लिए एडमिन या मॉडरेटर अनुमतियाँ आवश्यक हैं, या बातचीत प्रमाणित उपयोगकर्ता द्वारा बनाई गई होनी चाहिए।

#### एंडपॉइंट

[inline-code-attrs-start title = 'DELETE एंडपॉइंट'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### पैरामीटर

`tenantId` (पाथ पैरामीटर, आवश्यक) आपकी FastComments Tenant ID है।

`chatId` (पाथ पैरामीटर, आवश्यक) हटाने के लिए बातचीत की ID है।

`broadcastId` (अनुरोध बॉडी, आवश्यक) लाइव सिंक्रोनाइज़ेशन के लिए एक UUID है।

#### उदाहरण अनुरोध

[inline-code-attrs-start title = 'DELETE अनुरोध उदाहरण'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### उदाहरण प्रतिक्रिया

[inline-code-attrs-start title = 'DELETE प्रतिक्रिया उदाहरण'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### दर सीमित करना

ये एंडपॉइंट मानक FastComments API दर सीमा के अधीन हैं। दर सीमा मिडलवेयर सामान्य उपयोग पैटर्न की अनुमति देते हुए दुरुपयोग को रोकने के लिए प्रति-टेनेंट लागू होता है।

### त्रुटि प्रतिक्रियाएँ

सभी एंडपॉइंट मानक HTTP स्थिति कोड लौटाते हैं। 400 प्रतिक्रिया अमान्य अनुरोध पैरामीटर दर्शाती है। 401 प्रतिक्रिया का अर्थ है प्रमाणीकरण विफल। 403 प्रतिक्रिया अपर्याप्त अनुमतियाँ दर्शाती है। 404 प्रतिक्रिया का अर्थ है कि बातचीत नहीं मिली। 429 प्रतिक्रिया दर सीमा पार होने का संकेत देती है।

त्रुटि प्रतिक्रियाओं में विवरण के साथ एक JSON बॉडी शामिल होती है:

[inline-code-attrs-start title = 'त्रुटि प्रतिक्रिया उदाहरण'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### उपयोग ट्रैकिंग

बातचीत बनाना आपके `conversationCreateCount` उपयोग मीट्रिक को बढ़ाता है। सभी WebSocket सिंक गतिविधि `pubSubMessageCount` और `pubSubBandwidth` को बढ़ाती है। आप उपयोग विश्लेषण के अंतर्गत अपने FastComments डैशबोर्ड में इन मीट्रिक्स की निगरानी कर सकते हैं।
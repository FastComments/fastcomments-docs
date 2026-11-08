अनुक्रमणिका (index) पेज पर, प्रत्येक पंक्ति के लिए एक comment-count विजेट रेंडर न करें। इसका मतलब है प्रत्येक पोस्ट के लिए एक अनुरोध। बल्क काउंट का उपयोग करें, जो पूरे पेज के लिए एक ही अनुरोध लेता है।

प्रत्येक पंक्ति को उसके थ्रेड द्वारा उपयोग किए जाने वाले `urlId` से चिह्नित करें, फिर बल्क विजेट को एक बार लोड करें:

[inline-code-attrs-start title = 'अनुक्रमणिका पर बल्क टिप्पणी गणना'; type='javascript' inline-code-attrs-end]
[inline-code-start]
<ul>
  {posts.map((post) => (
    <li>
      <a href={post.slug}>{post.title}</a>{" "}
      <span class="fast-comments-count" data-fast-comments-url-id={post.slug}></span>
    </li>
  ))}
</ul>

<script
  dangerouslySetInnerHTML={{
    __html: `window.FastCommentsBulkCountConfig = ${
      JSON.stringify({ tenantId: TENANT_ID })
    };`,
  }}
/>
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>
[inline-code-end]

स्क्रिप्ट पेज पर हर `.fast-comments-count` एलिमेंट को ढूंढती है और उसमें उसकी गणना भरती है।

`data-fast-comments-url-id` को उस पोस्ट के कमेंट विजेट द्वारा उपयोग किए जाने वाले `urlId` से मेल खाना चाहिए। यदि विजेट स्लग का उपयोग करता है, तो मार्कर स्लग का उपयोग करता है। बेमेल होने पर ऐसे थ्रेड पर शून्य दिखाई देगा जिसमें टिप्पणियाँ मौजूद हैं।

स्क्रिप्ट `window.FastCommentsBulkCountConfig` के लिए पोलिंग करती है, इसलिए यह मायने नहीं रखता कि आपने कॉन्फ़िग को स्क्रिप्ट टैग से पहले या बाद में सेट किया है।
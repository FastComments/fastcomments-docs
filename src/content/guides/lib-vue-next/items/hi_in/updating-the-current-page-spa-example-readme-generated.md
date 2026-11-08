FastComments में हम आर्टिकल आईडी, या पेज जिससे कमेंट्स जुड़े होते हैं, को URL ID कहते हैं क्योंकि यह एक url या ID हो सकता है।
URL ID को निम्नलिखित तरीके से परिभाषित करें। कंपोनेंट config ऑब्जेक्ट में बदलावों को देखता है और रीलोड होगा, इसलिए आप URL ID अपडेट कर सकते हैं।

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### अकाउंट रीजन (ध्यान दें: EU ग्राहक)

यदि आपका अकाउंट EU में स्थित है, तो विजेट कॉन्फ़िगरेशन में `region = 'eu'` सेट करें, उदाहरण के लिए:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

अन्यथा, आपको `region` परिभाषित करने की आवश्यकता नहीं है।
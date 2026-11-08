Vue से बनी अपनी वेबसाइट पर कमेंट्स जोड़ने के लिए, हमारी Vue लाइब्रेरी NPM पर <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">यहाँ</a> पा सकते हैं।

इसके अतिरिक्त, एक vue-next लाइब्रेरी NPM पर <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">यहाँ</a> उपलब्ध है।

सोर्स कोड <a href="https://github.com/FastComments" target="_blank">GitHub</a> पर पाया जा सकता है।

FastComments Vue कमेंटिंग विजेट VanillaJS वाले की सभी सुविधाओं को सपोर्ट करता है - लाइव कमेंटिंग, sso, और भी बहुत कुछ।

नीचे दिए गए निर्देश Vue 3 के लिए हैं क्योंकि यह काफी समय से बाज़ार में है, हालाँकि FastComments `fastcomments-vue` लाइब्रेरी के ज़रिए Vue 2 को भी सपोर्ट करता है।

[inline-code-attrs-start title = 'NPM के ज़रिए FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Yarn के ज़रिए FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<template>
  <img alt="Vue logo" src="./assets/logo.png">
    <fast-comments v-bind:config="{tenantId: 'demo'}"/>
</template>

<script>
import {FastComments} from 'fastcomments-vue-next';

export default {
  name: 'App',
  components: {
    FastComments
  }
}
</script>
[inline-code-end]

यदि आप EU में हैं, तो आप `region` को `EU` सेट करना चाहेंगे:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue` और `fastcomments-vue-next` लाइब्रेरीज़ VanillaJS कमेंटिंग विजेट के समान कॉन्फ़िगरेशन सपोर्ट करती हैं।

Vue कंपोनेंट द्वारा सपोर्टेड कॉन्फ़िगरेशन आप <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">यहाँ</a> पा सकते हैं।
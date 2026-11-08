Angular से बनी साइट पर कमेंट जोड़ने के लिए, आप हमारी Angular लाइब्रेरी NPM पर <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">यहाँ</a> पा सकते हैं।

FastComments Angular कमेंटिंग विजेट VanillaJS वाले की सभी सुविधाओं को सपोर्ट करता है - लाइव कमेंटिंग, sso, और भी बहुत कुछ।

आपको fastcomments-typescript की आवश्यकता होगी, जो एक peer dependency है। कृपया सुनिश्चित करें कि यह आपके TypeScript compilation में शामिल हो।
भविष्य में, यह peer dependency @types/fastcomments में स्थानांतरित कर दी जाएगी, जिससे यह इंस्टॉलेशन आसान हो जाएगा।

[inline-code-attrs-start title = 'NPM के माध्यम से FastComments Angular'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer dependency को आपकी tsconfig.json फ़ाइल में जोड़ा जाना चाहिए, उदाहरण के लिए:

[inline-code-attrs-start title = 'fastcomments-typescript peer dependency जोड़ना'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

फिर, अपने एप्लिकेशन में `FastCommentsModule` जोड़ें:

[inline-code-attrs-start title = 'अपने ऐप में मॉड्यूल जोड़ें'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppComponent } from './app.component';
import { FastCommentsModule } from 'ngx-fastcomments';

@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    BrowserModule,
    FastCommentsModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
[inline-code-end]

## उपयोग

शुरू करने के लिए, हम डेमो टेनेंट के लिए एक config ऑब्जेक्ट पास करते हैं:

[inline-code-attrs-start title = 'उपयोग - इनलाइन कॉन्फ़िगरेशन'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

चूंकि कॉन्फ़िगरेशन काफी जटिल हो सकता है, हम एक ऑब्जेक्ट रेफरेंस पास कर सकते हैं:

[inline-code-attrs-start title = 'उपयोग - कॉन्फ़िगरेशन के लिए ऑब्जेक्ट पास करना'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'उपयोग - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

विजेट change detection का उपयोग करता है, इसलिए कॉन्फ़िगरेशन ऑब्जेक्ट की किसी भी प्रॉपर्टी को बदलने से यह रीलोड हो जाएगा।

Angular कंपोनेंट द्वारा सपोर्टेड कॉन्फ़िगरेशन आप <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">यहाँ</a> देख सकते हैं।
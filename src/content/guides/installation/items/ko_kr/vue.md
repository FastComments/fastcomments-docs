Vue로 구축된 웹사이트에 댓글을 추가하려면 NPM에서 우리의 Vue 라이브러리를 <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">여기</a>에서 찾을 수 있습니다.

추가로, vue-next 라이브러리는 NPM에서 <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">여기</a>에서 확인할 수 있습니다.

소스 코드는 <a href="https://github.com/FastComments" target="_blank">GitHub</a>에서 확인할 수 있습니다.

FastComments Vue 댓글 위젯은 VanillaJS 버전과 동일한 모든 기능(실시간 댓글, SSO 등)을 지원합니다.

아래 지침은 Vue 3을 기준으로 합니다(이미 출시된 지 시간이 지났기 때문). 그러나 FastComments는 `fastcomments-vue` 라이브러리를 통해 Vue 2도 지원합니다.

[inline-code-attrs-start title = 'NPM을 통한 FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Yarn을 통한 FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

EU에 있으면 `region`을 `EU`로 설정해야 합니다:

[inline-code-attrs-start title = 'FastComments Vue - EU 설정'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue` 및 `fastcomments-vue-next` 라이브러리는 VanillaJS 댓글 위젯과 동일한 구성을 지원합니다.

Vue 컴포넌트가 지원하는 구성은 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">여기</a>에서 확인할 수 있습니다.
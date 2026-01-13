Vue 라이브러리는 NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">여기</a>에서 찾을 수 있습니다.

또한 vue-next 라이브러리는 NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">여기</a>에 있습니다

소스 코드는 <a href="https://github.com/FastComments" target="_blank">GitHub</a>에서 찾을 수 있습니다.

FastComments Vue 댓글 위젯은 VanillaJS 버전과 동일한 모든 기능(실시간 댓글, SSO 등)을 지원합니다.

아래 지침은 Vue 3용입니다(출시된 지 오래되었기 때문). 그러나 FastComments는 `fastcomments-vue` 라이브러리를 통해 Vue 2도 지원합니다.

[inline-code-attrs-start title = 'FastComments Vue (NPM 사용)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue (Yarn 사용)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
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

EU에 계신 경우 `region`을 `EU`로 설정해야 합니다:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue`와 `fastcomments-vue-next` 라이브러리는 VanillaJS 댓글 위젯과 동일한 구성을 지원합니다.

Vue 컴포넌트가 지원하는 구성은 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">여기</a>에서 찾을 수 있습니다.

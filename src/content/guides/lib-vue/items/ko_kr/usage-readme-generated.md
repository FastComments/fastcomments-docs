### 댓글 위젯

FastCommentsVue 컴포넌트에는 라이브 FastComments 댓글 위젯이 포함되어 있습니다.

아래의 "demo"를 귀하의 "tenantId"로 바꾸세요 - FastComments 관리자 영역의 [here](https://fastcomments.com/auth/my-account/api)에서 확인할 수 있습니다.

이 위젯은 많은 옵션을 지원합니다 - FastCommentsConfig는 [here](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts)에서 확인하세요.

```vue
<script lang="ts">
import Vue from 'vue';
import FastCommentsVue from 'fastcomments-vue';

export default Vue.extend({
  name: 'ServeDev',
  components: {
    FastCommentsVue
  }
});
</script>

<template>
  <div id="app">
    <fast-comments-vue v-bind:config="{tenantId: 'demo'}" />
  </div>
</template>
```
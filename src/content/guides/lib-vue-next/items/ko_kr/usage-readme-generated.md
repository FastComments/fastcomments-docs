---
### 댓글 위젯

FastCommentsVueNext 컴포넌트는 라이브 FastComments 댓글 위젯을 포함합니다.

아래의 "demo"를 귀하의 "tenantId"로 바꾸세요 - FastComments 관리자 영역에서 [여기](https://fastcomments.com/auth/my-account/api)에서 확인할 수 있습니다.

이 위젯은 많은 옵션을 지원합니다 - FastCommentsConfig는 [여기](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14)를 참조하세요.

```vue
<template>
  <FastComments v-bind:config="{tenantId: 'demo'}" />
</template>
<script>
import { FastComments } from 'fastcomments-vue-next'
export default {
  name: 'FastCommentsExample',
  components: {
    FastComments
  }
}
</script>
```
---
---
### Виджет комментариев

Компонент FastCommentsVueNext содержит живой виджет комментариев FastComments.

Замените "demo" ниже на ваш "tenantId" — доступен [here](https://fastcomments.com/auth/my-account/api) в административной панели FastComments.

Виджет поддерживает множество опций — смотрите FastCommentsConfig [here](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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
### Widżet komentarzy

Komponent FastCommentsVueNext zawiera widżet komentarzy FastComments działający na żywo.

Zamień "demo" poniżej na swój "tenantId" - dostępny [tutaj](https://fastcomments.com/auth/my-account/api) w obszarze administracyjnym FastComments.

Widżet obsługuje wiele opcji - zobacz FastCommentsConfig [tutaj](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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
```svelte
<script lang="ts">
  import CommentWidget from "fastcomments-svelte/CommentWidget.svelte";
  import type { FastCommentsCommentWidgetConfig } from "fastcomments-typescript";

  let config: FastCommentsCommentWidgetConfig = {
    tenantId: "demo",
    urlId: "my-page"
  };
</script>

<CommentWidget config={config} />
```
---
### Уиджет за коментари

```blade
<x-fastcomments />

\{{-- С опции --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Чат на живо

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Брой коментари

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
---
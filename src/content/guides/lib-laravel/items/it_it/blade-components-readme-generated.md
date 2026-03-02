---
### Widget dei commenti

```blade
<x-fastcomments />

\{{-- Con opzioni --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Chat dal vivo

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Conteggio commenti

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
---
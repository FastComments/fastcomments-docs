---
### Widget za komentarje

```blade
<x-fastcomments />

{{-- Z možnostmi --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Klepet v živo

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Število komentarjev

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
---
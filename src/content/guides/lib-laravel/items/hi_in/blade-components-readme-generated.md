### कमेंट विजेट

```blade
<x-fastcomments />

\{{-- विकल्पों के साथ --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### लाइव चैट

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### कमेंट काउंट

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
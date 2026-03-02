### Kommentar-widget

```blade
<x-fastcomments />

{{-- Med indstillinger --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Livechat

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Antal kommentarer

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
### Видгет коментара

```blade
<x-fastcomments />

{{-- Са опцијама --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Разговор уживо

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Број коментара

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
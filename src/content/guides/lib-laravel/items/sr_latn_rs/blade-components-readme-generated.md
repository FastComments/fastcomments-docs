### Vidžet za komentare

```blade
<x-fastcomments />

\{{-- Sa opcijama --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Ćaskanje uživo

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Broj komentara

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
### Yorum Bileşeni

```blade
<x-fastcomments />

\{{-- Seçeneklerle --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Canlı Sohbet

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Yorum Sayısı

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
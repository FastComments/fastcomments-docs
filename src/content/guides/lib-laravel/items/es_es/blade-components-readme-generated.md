### Widget de comentarios

```blade
<x-fastcomments />

\{{-- Con opciones --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Chat en vivo

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Recuento de comentarios

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
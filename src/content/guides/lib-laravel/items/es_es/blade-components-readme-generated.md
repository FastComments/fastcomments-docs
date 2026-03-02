---
### Widget de Comentarios

```blade
<x-fastcomments />

{{-- Con opciones --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Chat en Vivo

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Contador de Comentarios

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
---
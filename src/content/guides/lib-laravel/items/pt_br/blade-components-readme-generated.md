### Widget de Comentários

```blade
<x-fastcomments />

{{-- Com opções --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### Chat ao Vivo

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### Contagem de Comentários

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
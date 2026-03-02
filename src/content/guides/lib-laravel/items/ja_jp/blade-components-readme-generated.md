### コメントウィジェット

```blade
<x-fastcomments />

\{{-- オプション付き --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### ライブチャット

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### コメント数

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
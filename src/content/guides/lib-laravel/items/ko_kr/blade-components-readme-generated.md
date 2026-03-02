### 댓글 위젯

```blade
<x-fastcomments />

{{-- 옵션 포함 --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### 라이브 채팅

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### 댓글 수

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
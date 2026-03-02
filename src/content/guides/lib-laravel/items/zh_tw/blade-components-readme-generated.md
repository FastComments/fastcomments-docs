### 評論小工具

```blade
<x-fastcomments />

{{-- 使用選項 --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### 即時聊天

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### 評論計數

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
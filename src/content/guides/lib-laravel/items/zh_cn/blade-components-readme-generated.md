---
### 评论小部件

```blade
<x-fastcomments />

\{{-- With options --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### 实时聊天

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### 评论计数

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
---
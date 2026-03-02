### ווידג'ט תגובות

```blade
<x-fastcomments />

{{-- עם אפשרויות --}}
<x-fastcomments
    url-id="my-page-id"
    url="https://example.com/my-page"
    locale="en_us"
    :has-dark-background="true"
    default-sort-direction="MR"
/>
```

### צ'אט חי

```blade
<x-fastcomments-live-chat url-id="chat-room-1" />
```

### ספירת תגובות

```blade
<x-fastcomments-comment-count url-id="my-page-id" />
<x-fastcomments-comment-count url-id="my-page-id" :number-only="true" />
```
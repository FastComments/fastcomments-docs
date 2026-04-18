```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

将 `"demo"` 替换为您的 FastComments 租户 ID。该组件已经声明了 'use client'，因此您可以从服务器组件渲染它。
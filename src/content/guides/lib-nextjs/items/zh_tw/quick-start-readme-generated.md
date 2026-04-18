```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

將 `"demo"` 替換為您的 FastComments 租戶 ID。該元件已經宣告 `'use client'`，因此您可以從伺服器元件中呈現它。
```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

`"demo"` をあなたの FastComments テナントIDに置き換えてください。コンポーネントは既に `'use client'` を宣言しているため、サーバーコンポーネントからレンダリングできます。
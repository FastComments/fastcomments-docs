```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Replace `"demo"` with your FastComments tenant ID. The component already declares `'use client'`, so you can render it from a server component.
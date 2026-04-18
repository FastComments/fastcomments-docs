```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Sostituisci `"demo"` con il tuo ID tenant di FastComments. Il componente dichiara già `'use client'`, quindi puoi renderizzarlo da un componente server.
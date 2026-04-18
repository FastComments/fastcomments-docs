```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Ersetzen Sie "demo" durch Ihre FastComments-Tenant-ID. Die Komponente deklariert bereits 'use client', sodass Sie sie aus einer Serverkomponente rendern können.
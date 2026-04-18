```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Sustituye `"demo"` por el ID de tenant de FastComments. El componente ya declara `'use client'`, por lo que puedes renderizarlo desde un componente del servidor.
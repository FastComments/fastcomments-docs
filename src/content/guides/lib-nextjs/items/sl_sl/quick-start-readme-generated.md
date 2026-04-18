```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Zamenjajte `"demo"` z ID-jem vašega FastComments najemnika. Komponenta že deklarira `'use client'`, zato jo lahko upodabljate iz strežniške komponente.
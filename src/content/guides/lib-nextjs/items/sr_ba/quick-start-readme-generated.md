```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Zamijenite `"demo"` sa vašim FastComments tenant ID-jem. Komponenta već deklarira `'use client'`, pa je možete renderovati iz server komponente.
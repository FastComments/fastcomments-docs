```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Zamijenite "demo" sa svojim FastComments tenant ID-jem. Komponenta već deklarira 'use client', pa je možete renderirati iz serverske komponente.
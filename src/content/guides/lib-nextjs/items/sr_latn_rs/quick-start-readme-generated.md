```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Zamenite "demo" svojim FastComments tenant ID-jem. Komponenta već deklariše 'use client', tako da je možete renderovati iz server komponente.
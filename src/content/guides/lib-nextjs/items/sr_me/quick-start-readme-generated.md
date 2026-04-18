```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Zamijenite `"demo"` sa vašim FastComments tenant ID-om. Komponenta već deklariše `'use client'`, tako da je možete renderovati iz server komponente.
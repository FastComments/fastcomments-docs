```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Substitua `"demo"` pelo ID do seu tenant do FastComments. O componente já declara `'use client'`, então você pode renderizá-lo a partir de um componente de servidor.
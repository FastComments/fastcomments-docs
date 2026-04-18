```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Remplacez `"demo"` par l'ID de locataire FastComments. Le composant déclare déjà 'use client', vous pouvez donc le rendre depuis un composant serveur.
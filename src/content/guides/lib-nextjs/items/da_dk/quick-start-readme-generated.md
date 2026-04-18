```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Erstat "demo" med dit FastComments tenant ID. Komponenten erklærer allerede 'use client', så du kan gengive den fra en serverkomponent.
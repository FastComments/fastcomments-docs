```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Vervang `"demo"` door je FastComments tenant ID. De component geeft al `'use client'` aan, dus je kunt deze vanuit een servercomponent renderen.
```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

החלף את `"demo"` ב-tenant ID של FastComments שלך. הקומפוננטה כבר מצהירה `'use client'`, לכן תוכל להציג אותה מתוך קומפוננטת שרת.
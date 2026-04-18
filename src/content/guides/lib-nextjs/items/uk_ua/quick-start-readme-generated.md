```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Замініть "demo" на свій FastComments tenant ID. Компонент вже оголошує 'use client', тому ви можете рендерити його з серверного компонента.
```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Замените "demo" са вашим FastComments tenant ID-ом. Компонента већ декларише 'use client', тако да је можете рендеровати из серверске компоненте.
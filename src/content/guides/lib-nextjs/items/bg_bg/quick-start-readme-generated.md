```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Заменете `"demo"` с вашия FastComments tenant ID. Компонентът вече декларира `'use client'`, така че можете да го рендерирате от сървърен компонент.
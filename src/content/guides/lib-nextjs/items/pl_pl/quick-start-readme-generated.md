```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Zastąp "demo" identyfikatorem swojej instancji (tenant) FastComments. Komponent już deklaruje 'use client', więc możesz renderować go z komponentu serwerowego.
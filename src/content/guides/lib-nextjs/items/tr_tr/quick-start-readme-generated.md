```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

"demo" öğesini FastComments kiracı kimliğinizle değiştirin. Bileşen zaten 'use client' bildirimini içeriyor, bu yüzden onu bir sunucu bileşeninden render edebilirsiniz.
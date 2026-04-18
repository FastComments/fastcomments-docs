```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Αντικαταστήστε το "demo" με το tenant ID του FastComments σας. Το συστατικό δηλώνει ήδη 'use client', οπότε μπορείτε να το αποδώσετε από ένα συστατικό διακομιστή.
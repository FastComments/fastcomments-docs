```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

`"demo"` को अपनी FastComments tenant ID से बदलें। कंपोनेंट में `'use client'` पहले से डिक्लेयर है, इसलिए आप इसे सर्वर कंपोनेंट से भी रेंडर कर सकते हैं।
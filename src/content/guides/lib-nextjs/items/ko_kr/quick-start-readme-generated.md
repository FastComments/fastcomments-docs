```tsx
// app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

"demo"를 FastComments 테넌트 ID로 바꿔주세요. 컴포넌트는 이미 'use client'를 선언하고 있으므로 서버 컴포넌트에서 렌더링할 수 있습니다.
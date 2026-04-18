```tsx
// файл app/page.tsx
import { FastComments } from 'fastcomments-nextjs';

export default function Page() {
    return <FastComments tenantId="demo" />;
}
```

Замените `"demo"` на ваш FastComments tenant ID. Компонент уже объявляет `'use client'`, поэтому вы можете рендерить его из серверного компонента.
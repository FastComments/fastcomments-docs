Solid אינו עוקב אוטומטית אחר שינויים עמוקים באובייקטים שרירותיים, ולכן שינויי קונפיגורציה לאחר הרינדור הראשון חייבים להידחף באופן מפורש. כל ווידג'ט מקבל `apiRef` שמחזיר handle; קראו ל-`handle.update(partial)` מתוך `createEffect` כדי להניע את הריאקטיביות:

```tsx
import { createEffect, createSignal } from 'solid-js';
import { FastCommentsCommentWidget, type FastCommentsCommentWidgetHandle } from 'fastcomments-solidjs';

export default function Paginated() {
  const [page, setPage] = createSignal(0);
  let handle: FastCommentsCommentWidgetHandle | undefined;
  createEffect(() => handle?.update({ urlId: `product-${page()}` }));

  return (
    <>
      <button onClick={() => setPage(page() + 1)}>next</button>
      <FastCommentsCommentWidget
        apiRef={(h) => (handle = h)}
        tenantId="demo"
        urlId={`product-${page()}`}
      />
    </>
  );
}
```

`update()` בטוח לקריאה בכל עת:
- לפני שהסקריפט נטען: ה-partial נשמר בצד ומיושם בעת האתחול.
- במהלך אתחול אסינכרוני (reviews-summary, user-activity-feed): ה-partial נכנס לתור ומיושם כשה-callback מסתיים.
- לאחר האתחול: הוא מועבר ישירות למתודת `.update()` של הווידג'ט החי.

### API של imperative handle

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // המופע החי העדכני ביותר (או null לפני ה-mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // מופעל פעם אחת כשהמופע מוכן
  update: (partial: Partial<Config>) => void; // מיזוג ודחיפה של הקונפיגורציה
}
```

השתמשו ב-`getInstance()` עבור פעולות imperative שאינן מכוסות על ידי `.update()`, למשל `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
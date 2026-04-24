Solid לא עוקבת באופן אוטומטי אחרי שינויים עמוקים על אובייקטים אקראיים, כך
ששינויים בקונפיגורציה לאחר ה-render הראשון חייבים להיות מועברים במפורש. כל ווידג'ט
מקבל `apiRef` שמחזיר handle; קראו ל-`handle.update(partial)` מתוך
`createEffect` כדי להניע ריאקטיביות:

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
- Before the script has loaded: the partial is stashed and applied at init.
- During an async init (reviews-summary, user-activity-feed): the partial is queued and applied when the callback resolves.
- After init: it forwards straight to the live widget's `.update()` method.

### API אימפרטיבי של ה-handle

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // המופע החי האחרון (או null לפני ההרכבה)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // נקרא פעם אחת כשהמופע מוכן
  update: (partial: Partial<Config>) => void; // מיזוג ודחיפת הקונפיגורציה
}
```

Use `getInstance()` for imperative actions that aren't covered by `.update()`, e.g. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
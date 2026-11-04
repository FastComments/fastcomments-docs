Solid nie śledzi automatycznie głębokich mutacji na dowolnych obiektach, więc zmiany konfiguracji po pierwszym renderze muszą być przekazywane jawnie. Każdy widget przyjmuje `apiRef`, który zwraca uchwyt; wywołaj `handle.update(partial)` z poziomu `createEffect`, aby napędzać reaktywność:

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

`update()` można bezpiecznie wywołać w dowolnym momencie:
- Zanim skrypt się załaduje: partial jest przechowywany i stosowany przy inicjalizacji.
- Podczas asynchronicznej inicjalizacji (reviews-summary, user-activity-feed): partial jest kolejkowany i stosowany, gdy callback się zakończy.
- Po inicjalizacji: jest przekazywany bezpośrednio do metody `.update()` działającego widgetu.

### Imperatywne API uchwytu

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // latest live instance (or null before mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // fires once instance is ready
  update: (partial: Partial<Config>) => void; // merge-and-push config
}
```

Użyj `getInstance()` dla akcji imperatywnych, które nie są objęte przez `.update()`, np. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
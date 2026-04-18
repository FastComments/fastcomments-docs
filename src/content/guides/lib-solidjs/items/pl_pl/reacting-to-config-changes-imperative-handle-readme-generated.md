Solid nie śledzi automatycznie głębokich mutacji w dowolnych obiektach, więc zmiany konfiguracji po pierwszym renderze muszą być zastosowane jawnie. Każdy widget akceptuje `apiRef`, który zwraca uchwyt; wywołaj `handle.update(partial)` z `createEffect`, aby napędzać reaktywność:

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

`update()` is safe to call at any time:
- Przed załadowaniem skryptu: partial jest odłożony i zastosowany podczas inicjalizacji.
- Podczas asynchronicznej inicjalizacji (reviews-summary, user-activity-feed): partial jest umieszczony w kolejce i zastosowany, gdy callback się zakończy.
- Po inicjalizacji: jest przekazywany bezpośrednio do metody `.update()` działającego widgetu.

### Imperative handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // ostatnia aktywna instancja (lub null przed zamontowaniem)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // wywoływane, gdy instancja jest gotowa
  update: (partial: Partial<Config>) => void; // scal i zastosuj konfigurację
}
```

Use `getInstance()` for imperative actions that aren't covered by `.update()`, e.g. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
Solid verfolgt nicht automatisch tiefe Mutationen beliebiger Objekte, daher müssen config-Änderungen nach dem ersten Render explizit angewendet werden. Jedes Widget akzeptiert ein `apiRef`, das ein Handle zurückgibt; rufen Sie `handle.update(partial)` aus einem `createEffect` auf, um Reaktivität zu steuern:

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

`update()` kann jederzeit sicher aufgerufen werden:
- Bevor das Skript geladen ist: das partial wird zwischengespeichert und beim init angewendet.
- Während einer asynchronen Init (reviews-summary, user-activity-feed): das partial wird in die Warteschlange gestellt und angewendet, wenn der callback aufgelöst wird.
- Nach dem init: es wird direkt an die live widget's `.update()`-Methode weitergeleitet.

### Imperative Handle-API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // aktuellste Live-Instanz (oder null vor mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // wird ausgelöst, sobald die Instanz bereit ist
  update: (partial: Partial<Config>) => void; // config zusammenführen und pushen
}
```

Verwenden Sie `getInstance()` für imperative Aktionen, die nicht durch `.update()` abgedeckt sind, z. B. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
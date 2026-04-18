Solid no realiza un seguimiento automático de mutaciones profundas en objetos arbitrarios, por lo que
los cambios de configuración después del primer renderizado deben enviarse explícitamente. Every widget
acepta un `apiRef` que devuelve un handle; llame a `handle.update(partial)` desde
un `createEffect` para impulsar la reactividad:

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

`update()` es seguro de llamar en cualquier momento:
- Before the script has loaded: the partial is stashed and applied at init.
- During an async init (reviews-summary, user-activity-feed): the partial is queued and applied when the callback resolves.
- After init: it forwards straight to the live widget's `.update()` method.

### API imperativa del handle

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // última instancia en vivo (o null antes del montaje)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // se dispara una vez que la instancia está lista
  update: (partial: Partial<Config>) => void; // fusiona y empuja la configuración
}
```

Utilice `getInstance()` para acciones imperativas que no estén cubiertas por `.update()`, p. ej. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
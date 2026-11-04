Solid no rastrea automáticamente las mutaciones profundas en objetos arbitrarios, por lo que
los cambios de configuración después del primer renderizado deben enviarse explícitamente. Cada widget
acepta un `apiRef` que devuelve un handle; llama a `handle.update(partial)` desde
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
- Antes de que el script se haya cargado: el partial se guarda temporalmente y se aplica en la inicialización.
- Durante una inicialización asíncrona (reviews-summary, user-activity-feed): el partial se encola y se aplica cuando el callback se resuelve.
- Después de la inicialización: se reenvía directamente al método `.update()` del widget en vivo.

### API del handle imperativo

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // última instancia en vivo (o null antes del montaje)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // se dispara una vez cuando la instancia está lista
  update: (partial: Partial<Config>) => void; // fusiona y envía la configuración
}
```

Usa `getInstance()` para acciones imperativas que no están cubiertas por `.update()`, por ejemplo `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
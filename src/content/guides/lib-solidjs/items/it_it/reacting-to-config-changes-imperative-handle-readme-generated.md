Solid non traccia automaticamente le mutazioni profonde su oggetti arbitrari, quindi
le modifiche di configurazione dopo il primo rendering devono essere inviate esplicitamente. Ogni widget
accetta un `apiRef` che restituisce un handle; chiamare `handle.update(partial)` da
un `createEffect` per guidare la reattività:

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

`update()` è sicuro da chiamare in qualsiasi momento:
- Prima che lo script sia stato caricato: il partial viene accantonato e applicato all'inizializzazione.
- Durante un'inizializzazione asincrona (reviews-summary, user-activity-feed): il partial viene messo in coda e applicato quando la callback si risolve.
- Dopo l'inizializzazione: viene inoltrato direttamente al metodo `.update()` del widget attivo.

### API imperativa dell'handle

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // ultima istanza attiva (o null prima del montaggio)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // viene chiamato una volta che l'istanza è pronta
  update: (partial: Partial<Config>) => void; // unisce e invia la configurazione
}
```

Usa `getInstance()` per azioni imperative non coperte da `.update()`, ad es. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
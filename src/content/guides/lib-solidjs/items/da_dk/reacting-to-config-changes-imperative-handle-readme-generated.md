Solid sporer ikke automatisk dybe mutationer på vilkårlige objekter, så
konfigurationsændringer efter første render skal pushes eksplicit. Hver widget
accepterer en `apiRef`, der returnerer et handle; kald `handle.update(partial)` fra
en `createEffect` for at drive reaktivitet:

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

`update()` kan trygt kaldes når som helst:
- Før scriptet er indlæst: den partielle konfiguration gemmes og anvendes ved init.
- Under en asynkron init (reviews-summary, user-activity-feed): den partielle konfiguration sættes i kø og anvendes, når callback'en resolves.
- Efter init: den videresendes direkte til den live widgets `.update()`-metode.

### Imperativt handle-API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // seneste live-instans (eller null før mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // udløses én gang, når instansen er klar
  update: (partial: Partial<Config>) => void; // merge-and-push konfiguration
}
```

Brug `getInstance()` til imperative handlinger, der ikke er dækket af `.update()`, f.eks. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
---
Solid følger ikke automatisk dybe mutationer på vilkårlige objekter, så
konfigurationsændringer efter den første render skal skubbes eksplicit. Hvert widget
accepterer en `apiRef` som returnerer et håndtag; kald `handle.update(partial)` fra
et `createEffect` for at drive reaktivitet:

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

`update()` er sikkert at kalde når som helst:
- Before the script has loaded: the partial is stashed and applied at init.
- During an async init (reviews-summary, user-activity-feed): the partial is queued and applied when the callback resolves.
- After init: it forwards straight to the live widget's `.update()` method.

### Imperativt handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // seneste live-instans (eller null før mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // kaldes når instansen er klar
  update: (partial: Partial<Config>) => void; // slå config sammen og skub den
}
```

Brug `getInstance()` til imperative handlinger, som ikke dækkes af `.update()`, f.eks. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
---
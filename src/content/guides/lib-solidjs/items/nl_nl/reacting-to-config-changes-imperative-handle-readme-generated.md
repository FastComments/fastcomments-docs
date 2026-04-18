Solid volgt niet automatisch diepe mutaties op willekeurige objecten, dus
configwijzigingen na de eerste render moeten expliciet worden doorgegeven. Elke widget
accepteert een `apiRef` die een handle retourneert; roep `handle.update(partial)` aan vanuit
een `createEffect` om reactiviteit aan te sturen:

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

`update()` is veilig om op elk moment aan te roepen:
- Before the script has loaded: the partial is stashed and applied at init.
- During an async init (reviews-summary, user-activity-feed): the partial is queued and applied when the callback resolves.
- After init: it forwards straight to the live widget's `.update()` method.

### Imperatieve handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // latest live instance (or null before mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // fires once instance is ready
  update: (partial: Partial<Config>) => void; // merge-and-push config
}
```

Gebruik `getInstance()` voor imperatieve acties die niet door `.update()` worden afgedekt, bijv. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
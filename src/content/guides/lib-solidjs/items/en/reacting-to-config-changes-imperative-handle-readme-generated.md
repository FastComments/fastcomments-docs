Solid does not automatically track deep mutations on arbitrary objects, so
config changes after the first render must be pushed explicitly. Every widget
accepts an `apiRef` that returns a handle; call `handle.update(partial)` from
a `createEffect` to drive reactivity:

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
- Before the script has loaded: the partial is stashed and applied at init.
- During an async init (reviews-summary, user-activity-feed): the partial is queued and applied when the callback resolves.
- After init: it forwards straight to the live widget's `.update()` method.

### Imperative handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // latest live instance (or null before mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // fires once instance is ready
  update: (partial: Partial<Config>) => void; // merge-and-push config
}
```

Use `getInstance()` for imperative actions that aren't covered by `.update()`, e.g. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
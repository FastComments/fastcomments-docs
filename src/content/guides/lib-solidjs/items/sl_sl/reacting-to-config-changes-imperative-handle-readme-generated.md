Solid samodejno ne sledi globokim mutacijam poljubnih objektov, zato je treba spremembe konfiguracije po prvem renderiranju eksplicitno potisniti. Vsak widget sprejme `apiRef`, ki vrne ročaj; iz `createEffect` pokličite `handle.update(partial)`, da poganjate reaktivnost:

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

`update()` je varno klicati kadarkoli:
- Pred nalaganjem skripte: partial se shrani in uporabi pri init.
- Med async init (reviews-summary, user-activity-feed): partial se postavi v čakalno vrsto in uporabi, ko se callback razreši.
- Po init: ga posreduje neposredno metodi `.update()` živega widgeta.

### Imperativni API ročaja

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // najnovejša živa instanca (ali null pred mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // sproži se, ko je instanca pripravljena
  update: (partial: Partial<Config>) => void; // združi in potisni konfiguracijo
}
```

Uporabite `getInstance()` za imperativna dejanja, ki jih `.update()` ne pokriva, npr. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
Solid automatski ne prati duboke mutacije na proizvoljnim objektima, stoga promjene konfiguracije nakon prvog renderiranja moraju biti eksplicitno poslane. Svaki widget prihvaća `apiRef` koji vraća handle; pozovite `handle.update(partial)` iz `createEffect` kako biste potaknuli reaktivnost:

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

`update()` je sigurno pozvati u bilo kojem trenutku:
- Prije nego što se skripta učita: djelomični podaci se pohranjuju i primjenjuju pri inicijalizaciji.
- Tijekom asinhrone inicijalizacije (reviews-summary, user-activity-feed): djelomični podaci se stavljaju u red i primjenjuju kada se callback riješi.
- Nakon inicijalizacije: on ih prosljeđuje izravno metodi `.update()` aktivnog widgeta.

### Imperativni handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // zadnja aktivna instanca (ili null prije mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // poziva se jednom kada je instanca spremna
  update: (partial: Partial<Config>) => void; // spoji i pošalji konfiguraciju
}
```

Koristite `getInstance()` za imperativne akcije koje nisu pokrivene s `.update()`, npr. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
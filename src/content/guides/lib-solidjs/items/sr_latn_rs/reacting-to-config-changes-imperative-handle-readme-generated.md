Solid ne prati automatski duboke mutacije nad proizvoljnim objektima, tako da promene u config-u nakon prvog rendera moraju biti eksplicitno gurnute. Svaki widget prihvata `apiRef` koji vraća handle; pozovite `handle.update(partial)` iz `createEffect` da biste pokrenuli reaktivnost:

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

`update()` je bezbedno pozivati u bilo kom trenutku:
- Pre nego što se skript učita: partial se čuva i primenjuje pri init.
- Tokom asinhrone init faze (reviews-summary, user-activity-feed): partial se stavi u red i primeni kada se callback reši.
- Posle init-a: prosleđuje se direktno metodi `.update()` živog widgeta.

### Imperativni handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // najnovija aktivna instanca (ili null pre mount-a)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // poziva se jednom kada je instanca spremna
  update: (partial: Partial<Config>) => void; // spoji i pošalji konfiguraciju
}
```

Koristite `getInstance()` za imperativne akcije koje nisu obuhvaćene `.update()`, npr. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
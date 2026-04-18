Το Solid δεν παρακολουθεί αυτόματα βαθιές μεταβολές σε αυθαίρετα αντικείμενα, οπότε οι αλλαγές στο config μετά το πρώτο render πρέπει να προωθούνται ρητά. Κάθε widget δέχεται ένα `apiRef` που επιστρέφει ένα handle· καλέστε `handle.update(partial)` από ένα `createEffect` για να ενεργοποιήσετε την αντιδραστικότητα:

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

`update()` είναι ασφαλές να κληθεί οποιαδήποτε στιγμή:
- Πριν το script φορτωθεί: το partial αποθηκεύεται προσωρινά και εφαρμόζεται κατά το init.
- Κατά τη διάρκεια ενός ασύγχρονου init (reviews-summary, user-activity-feed): το partial τίθεται σε ουρά και εφαρμόζεται όταν η callback επιλυθεί.
- Μετά το init: προωθείται απευθείας στη μέθοδο `.update()` του ζωντανού widget.

### Επιτακτικό API χειρολαβής

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // πιο πρόσφατη ζωντανή instance (ή null πριν το mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // εκτελείται μόλις η instance είναι έτοιμη
  update: (partial: Partial<Config>) => void; // συγχώνευση και ώθηση config
}
```

Χρησιμοποιήστε `getInstance()` για επιτακτικές ενέργειες που δεν καλύπτονται από `.update()`, π.χ. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
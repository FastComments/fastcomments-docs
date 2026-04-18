Το Solid δεν παρακολουθεί αυτόματα βαθιές μεταβολές σε αυθαίρετα αντικείμενα, οπότε
οι αλλαγές στη διαμόρφωση μετά το πρώτο render πρέπει να ωθηθούν ρητά. Κάθε widget
δέχεται ένα `apiRef` που επιστρέφει ένα handle· καλέστε `handle.update(partial)` από
ένα `createEffect` για να οδηγήσετε την αντιδραστικότητα:

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
- Πριν φορτωθεί το script: το partial αποθηκεύεται και εφαρμόζεται κατά το init.
- Κατά τη διάρκεια ενός async init (reviews-summary, user-activity-feed): το partial τοποθετείται σε ουρά και εφαρμόζεται όταν το callback επιλυθεί.
- Μετά το init: προωθείται απευθείας στη μέθοδο `.update()` του live widget.

### Επιτακτικό API του handle

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // η πιο πρόσφατη ζωντανή παρουσία (ή null πριν από το mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // καλείται μια φορά όταν το instance είναι έτοιμο
  update: (partial: Partial<Config>) => void; // συγχώνευση και προώθηση του config
}
```

Χρησιμοποιήστε `getInstance()` για επιτακτικές ενέργειες που δεν καλύπτονται από `.update()`, π.χ. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
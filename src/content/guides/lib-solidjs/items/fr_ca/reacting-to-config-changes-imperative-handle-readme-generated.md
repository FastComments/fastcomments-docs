Solid ne suit pas automatiquement les mutations profondes sur des objets arbitraires, donc
les changements de configuration après le premier rendu doivent être poussés explicitement. Chaque widget
accepte un `apiRef` qui renvoie un handle ; appelez `handle.update(partial)` depuis un `createEffect` pour
alimenter la réactivité :

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

`update()` peut être appelé en toute sécurité à tout moment :
- Avant que le script ne soit chargé : le partial est mis en réserve et appliqué à l'initialisation.
- Pendant une initialisation asynchrone (reviews-summary, user-activity-feed) : le partial est mis en file d'attente et appliqué lorsque le callback se résout.
- Après l'initialisation : il est transmis directement à la méthode `.update()` du widget en direct.

### API impérative du handle

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // dernière instance en direct (ou null avant le montage)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // déclenché une fois que l'instance est prête
  update: (partial: Partial<Config>) => void; // fusionne et pousse la configuration
}
```

Utilisez `getInstance()` pour des actions impératives qui ne sont pas couvertes par `.update()`, p. ex. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
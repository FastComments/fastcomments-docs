Solid ne suit pas automatiquement les mutations profondes sur des objets arbitraires, donc
les modifications de config après le premier rendu doivent être poussées explicitement. Chaque widget
accepte un `apiRef` qui renvoie une handle ; appelez `handle.update(partial)` depuis
un `createEffect` pour piloter la réactivité :

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
- Avant que le script ne soit chargé : le partiel est mis en cache et appliqué à l'initialisation.
- Pendant une initialisation asynchrone (reviews-summary, user-activity-feed) : le partiel est mis en file d'attente et appliqué lorsque le callback est résolu.
- Après l'initialisation : il transmet directement à la méthode `.update()` du widget actif.

### API impérative du handle

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // dernière instance active (ou null avant le montage)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // se déclenche une fois que l'instance est prête
  update: (partial: Partial<Config>) => void; // fusionne et pousse la configuration
}
```

Utilisez `getInstance()` pour des actions impératives qui ne sont pas couvertes par `.update()`, par ex. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
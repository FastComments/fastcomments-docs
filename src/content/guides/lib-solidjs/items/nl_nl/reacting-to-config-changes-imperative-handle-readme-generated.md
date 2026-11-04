Solid houdt diepe mutaties op willekeurige objecten niet automatisch bij, dus
configuratiewijzigingen na de eerste render moeten expliciet worden doorgegeven. Elke widget
accepteert een `apiRef` die een handle teruggeeft; roep `handle.update(partial)` aan vanuit
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

`update()` kan op elk moment veilig worden aangeroepen:
- Voordat het script is geladen: het gedeeltelijke object wordt tijdelijk opgeslagen en toegepast bij de initialisatie.
- Tijdens een asynchrone initialisatie (reviews-summary, user-activity-feed): het gedeeltelijke object wordt in de wachtrij geplaatst en toegepast zodra de callback is afgerond.
- Na de initialisatie: het wordt rechtstreeks doorgestuurd naar de `.update()`-methode van de live widget.

### Imperatieve handle-API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // laatste live instantie (of null vóór het mounten)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // wordt eenmaal aangeroepen zodra de instantie gereed is
  update: (partial: Partial<Config>) => void; // configuratie samenvoegen en doorgeven
}
```

Gebruik `getInstance()` voor imperatieve acties die niet door `.update()` worden gedekt, bijv. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
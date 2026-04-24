Solid не прати аутоматски дубоке мутације на произвољним објектима, па измене конфигурације након првог рендеровања морају бити послате експлицитно. Сваки видгет прихвата `apiRef` који враћа руку; позовите `handle.update(partial)` из `createEffect` да покренете реактивност:

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

`update()` је безбедно позивати у било ком тренутку:
- Пре него што се скрипта учита: the partial се ставља на чување и примењује при иницијализацији.
- Током асинхроне инициализације (reviews-summary, user-activity-feed): the partial се ставља у ред и примењује када callback буде решен.
- Након иницијализације: директно прослеђује методи `.update()` живог видгета.

### Императивни handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // најновија активна инстанца (или null пре mount-а)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // покреће се једном када је инстанца спремна
  update: (partial: Partial<Config>) => void; // спаја и шаље конфигурацију
}
```

Користите `getInstance()` за императивне акције које нису покривене са `.update()`, нпр. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
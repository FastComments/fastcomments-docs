Solid аутоматски не прати дубоке мутције на произвољним објектима, тако да
промене конфигурације након првог рендера морају бити потиснуте експлицитно. Сваки видгет
прихвата `apiRef` који враћа хендл; позовите `handle.update(partial)` из
`createEffect` да бисте покренули реактивност:

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

`update()` је безбедно позивати у било које време:
- Пре него што се скрипта учита: делимични подаци се привремено чувају и примењују при иницијализацији.
- Током асинхроне иницијализације (reviews-summary, user-activity-feed): делимични подаци се стављају у ред и примењују када се повратни позив разреши.
- Након иницијализације: он директно прослеђује живом видгету његову `.update()` методу.

### Императивни API хендла

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // најновија жива инстанца (или null пре монтаже)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // позива се једном када је инстанца спремна
  update: (partial: Partial<Config>) => void; // споји и пошаље конфигурацију
}
```

Користите `getInstance()` за императивне акције које нису обухваћене `.update()`, нпр. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
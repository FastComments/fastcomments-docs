Solid не проследява автоматично дълбоки промени в произволни обекти, така че промените в конфигурацията след първото рендиране трябва да бъдат подадени явно. Всеки widget приема `apiRef`, който връща хендъл; извикайте `handle.update(partial)` от `createEffect`, за да задвижите реактивността:

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

`update()` е безопасно да се извиква по всяко време:
- Преди скриптът да е зареден: частичният обект се запазва и се прилага при инициализация.
- По време на асинхронна инициализация (reviews-summary, user-activity-feed): частичният обект се поставя в опашка и се прилага, когато callback-ът се изпълни.
- След инициализация: той директно препраща към `.update()` метода на живия widget.

### Императивен API на handle

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // най-нов жив екземпляр (или null преди монтиране)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // задейства се веднъж, когато екземплярът е готов
  update: (partial: Partial<Config>) => void; // слива и изпраща конфигурацията
}
```

Използвайте `getInstance()` за императивни действия, които не са покрити от `.update()`, напр. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
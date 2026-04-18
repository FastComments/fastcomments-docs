Solid rastgele nesnelerde derin değişiklikleri otomatik olarak takip etmez, bu nedenle
ilk render'dan sonra yapılandırma değişiklikleri açıkça gönderilmelidir. Her widget
bir handle döndüren bir `apiRef` kabul eder; reaktiviteyi tetiklemek için bir
`createEffect` içinde `handle.update(partial)` çağırın:

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

`update()` her zaman güvenle çağrılabilir:
- Script yüklenmeden önce: partial saklanır ve init sırasında uygulanır.
- Asenkron bir init sırasında (reviews-summary, user-activity-feed): partial kuyruğa alınır ve callback çözüldüğünde uygulanır.
- Init'ten sonra: canlı widget'ın `.update()` metoduna doğrudan iletilir.

### İmperatif handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // en son canlı örnek (veya mount öncesi null)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // örnek hazır olduğunda bir kez tetiklenir
  update: (partial: Partial<Config>) => void; // yapılandırmayı birleştir ve gönder
}
```

`.update()` tarafından kapsanmayan imperatif işlemler için `getInstance()` kullanın, örn. `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
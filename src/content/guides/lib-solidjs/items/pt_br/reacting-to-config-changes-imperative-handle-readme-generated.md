Solid não rastreia automaticamente mutações profundas em objetos arbitrários, portanto
as alterações de configuração após a primeira renderização devem ser aplicadas explicitamente. Every widget
aceita um `apiRef` que retorna um handle; chame `handle.update(partial)` a partir de
um `createEffect` para acionar a reatividade:

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

`update()` é seguro chamar a qualquer momento:
- Antes do script ser carregado: o partial é armazenado e aplicado na inicialização.
- Durante uma inicialização assíncrona (reviews-summary, user-activity-feed): o partial é enfileirado e aplicado quando o callback for resolvido.
- Após a inicialização: ele é encaminhado diretamente para o método `.update()` do widget ativo.

### API imperativa do handle

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // instância ao vivo mais recente (ou null antes do mount)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // acionado quando a instância estiver pronta
  update: (partial: Partial<Config>) => void; // mescla e envia a configuração
}
```

Use `getInstance()` para ações imperativas que não são cobertas por `.update()`, por exemplo, `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
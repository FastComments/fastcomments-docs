Solid अनियमित ऑब्जेक्ट्स पर गहरे mutations को स्वतः ट्रैक नहीं करता, इसलिए पहले render के बाद config में हुए बदलावों को स्पष्ट रूप से push करना होता है। हर widget एक `apiRef` स्वीकार करता है जो एक handle लौटाता है; reactivity चलाने के लिए `createEffect` से `handle.update(partial)` कॉल करें:

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

`update()` किसी भी समय कॉल करना सुरक्षित है:
- Script लोड होने से पहले: partial को stash कर लिया जाता है और init के समय apply किया जाता है।
- Async init के दौरान (reviews-summary, user-activity-feed): partial को queue किया जाता है और callback resolve होने पर apply किया जाता है।
- Init के बाद: यह सीधे live widget की `.update()` method पर forward हो जाता है।

### Imperative handle API

```ts
interface WidgetHandle<Config> {
  getInstance: () => WidgetInstance | null;   // नवीनतम live instance (mount से पहले null)
  onInstance: (cb: (instance: WidgetInstance) => void) => void; // instance तैयार होने पर एक बार चलता है
  update: (partial: Partial<Config>) => void; // config को merge और push करें
}
```

उन imperative क्रियाओं के लिए `getInstance()` का उपयोग करें जो `.update()` द्वारा कवर नहीं हैं, जैसे `openProfile`:

```tsx
const openProfile = () =>
  (handle?.getInstance() as { openProfile?: (o: { userId: string }) => void } | null)
    ?.openProfile?.({ userId: 'demo' });
```
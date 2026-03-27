---
Ова библиотека је потпуна react-native имплементација [FastComments](https://fastcomments.com).

Подржава коментарисање у реалном времену, ћаскање, теме (threads), емотиконе, обавештења, SSO, скинове и потпуно прилагођавање прослеђивањем stylesheet објекта. Сви assets такође се могу прилагодити, и подржава пребацивање различитих assets у зависности од dark mode-а.

Предност ове библиотеке је што је флексибилнија од wrapper-а `fastcomments-react-native`. Коментари се рендерују помоћу нативних компоненти уместо унутар webview-а. Напомена: `react-native-webview` је и даље потребан као транзитивна зависност rich text едитора (`@10play/tentap-editor`).

Све то ради на FastComments backend-у, тако да вам је потребно само да уградите UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Погледајте [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) за више примера.

Додајте ћаскање у реалном времену у вашу постојећу React Native апликацију, или чак изградите друштвену мрежу!
---
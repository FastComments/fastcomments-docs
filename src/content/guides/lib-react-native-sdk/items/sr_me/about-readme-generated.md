Ова библиотека је комплетна react-native имплементација [FastComments](https://fastcomments.com).

Подржава коментарисање уживо, ћаскање, нити, емотиконе, обавештења, SSO, скинове, и потпуно прилагођавање прослеђивањем објекта stylesheet. Сви ресурси
такође се могу прилагодити, и подржава пребацивање различитих ресурса у зависности од тамног режима.

Предност ове библиотеке је у томе што је флексибилнија од `fastcomments-react-native` wrapper-а. Коментари се приказују помоћу нативних компоненти уместо унутар webview-а.

Све ради на FastComments бекенду, па вам остаје само да уградите UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Погледајте [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) за више примера.

Додајте чат уживо у вашу постојећу React Native апликацију, или чак направите друштвену мрежу!
Ова библиотека је потпуна react-native имплементација [FastComments](https://fastcomments.com).

Подржава уживо коментарисање, ћаскање уживо, нитове, емотиконе, обавештења, SSO, теме и потпуно прилагођавање помоћу прослеђивања објекта stylesheet-а. Сви ресурси такође могу бити прилагођени, и подржава пребацивање различитих ресурса у зависности од тамног режима.

Предност ове библиотеке је што је флексибилнија од `fastcomments-react-native` wrapper-а. Коментари се приказују помоћу нативних компонената, уместо у webview-у.

Све ради на FastComments бекенду, тако да треба да уградите само UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Погледајте [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) за више примера.

Додајте ћаскање уживо у вашу постојећу React Native апликацију, или чак изградите друштвену мрежу!
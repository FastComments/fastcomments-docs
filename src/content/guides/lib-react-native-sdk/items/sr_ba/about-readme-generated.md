Ова библиотека је потпуна react-native имплементација [FastComments](https://fastcomments.com).

Подржава коментарисање уживо, чат, нитове, емотиконе, обавјештења, SSO, теме (skins) и потпуну прилагоду кроз просљеђивање stylesheet објекта. Сви асети
такође се могу прилагодити, и подржава пребацивање различитих асета засновано на тамном режиму.

Предност ове библиотеке је што је флексибилнија и не захтијева webview, као `fastcomments-react-native` wrapper.

Све ради на FastComments бекенду, тако да морате само да укључите UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Погледајте [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) за више примјера.

Додајте чат уживо у вашу постојећу React Native апликацију, или чак изградите друштвену мрежу!
Bu kütüphane, [FastComments](https://fastcomments.com) için eksiksiz bir react-native uygulamasıdır.

Canlı yorumlama, sohbet, thread'ler, emoticon'lar, bildirimler, SSO, skin'ler ve bir stylesheet nesnesi geçirerek tam özelleştirme desteği sağlar. Tüm varlıklar ayrıca özelleştirilebilir ve koyu moda göre farklı varlıkların değiştirilmesini destekler.

Bu kütüphanenin avantajı, `fastcomments-react-native` wrapper'ından daha esnek olmasıdır. Yorumlar bir webview içinde değil, native bileşenlerle render edilir.

Her şey FastComments backend üzerinde çalışır, bu yüzden yalnızca UI'yi entegre etmeniz yeterlidir:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Daha fazla örnek için [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src)'e bakın.

Mevcut React Native uygulamanıza canlı sohbet ekleyin veya hatta bir sosyal ağ oluşturun!
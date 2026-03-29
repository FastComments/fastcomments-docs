Bu kütüphane, [FastComments](https://fastcomments.com) için eksiksiz bir react-native uygulamasıdır.

Canlı yorumlama, sohbet, konular (threads), emotikonlar, bildirimler, SSO, temalar (skins) ve bir stylesheet nesnesi geçirerek tam özelleştirme desteği sağlar. Tüm varlıklar ayrıca özelleştirilebilir ve karanlık mod temelinde farklı varlıkların geçişini destekler.

Bu kütüphanenin avantajı, `fastcomments-react-native` sarıcısından daha esnek olmasıdır. Yorumlar bir webview içinde değil, yerel bileşenlerle render edilir. Not: `react-native-webview` zengin metin düzenleyicinin (`@10play/tentap-editor`) geçişli bir bağımlılığı olarak hâlâ gereklidir.

Her şey FastComments arka ucunda çalışır, bu yüzden yalnızca kullanıcı arayüzünü entegre etmeniz yeterlidir:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Daha fazla örnek için [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) adresine bakın.

Varolan React Native uygulamanıza canlı sohbet ekleyin veya hatta bir sosyal ağ oluşturun!
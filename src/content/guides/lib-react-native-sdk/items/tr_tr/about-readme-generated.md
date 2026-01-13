Bu kütüphane [FastComments](https://fastcomments.com) için eksiksiz bir react-native uygulamasıdır.

Canlı yorum yapma, sohbet, thread'ler, emoticon'lar, bildirimler, SSO, skin'ler ve bir stylesheet nesnesi geçirilerek tam özelleştirme destekler. Tüm varlıklar ayrıca özelleştirilebilir ve karanlık moda göre farklı varlıkların geçişini destekler.

Bu kütüphanenin faydası daha esnek olması ve `fastcomments-react-native` wrapper'ı gibi bir webview gerektirmemesidir.

Hepsi FastComments backend'inde çalışır, bu yüzden yalnızca kullanıcı arayüzünü entegre etmeniz yeterlidir:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Daha fazla örnek için [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) dizinine bakın.

Mevcut React Native uygulamanıza canlı sohbet ekleyin veya hatta bir sosyal ağ oluşturun!
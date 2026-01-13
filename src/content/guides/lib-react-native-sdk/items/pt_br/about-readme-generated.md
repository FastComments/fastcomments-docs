Esta biblioteca é uma implementação completa de react-native do [FastComments](https://fastcomments.com).

Ele suporta comentários ao vivo, chat, threads, emoticons, notificações, SSO, skins e personalização completa passando um objeto stylesheet. Todos os assets
também podem ser personalizados, e ele suporta alternar diferentes assets com base no modo escuro.

A vantagem desta biblioteca é que ela é mais flexível e não requer uma webview, como o wrapper `fastcomments-react-native`.

Tudo roda no backend do FastComments, então você só precisa incorporar a interface do usuário:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Veja [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) para mais exemplos.

Adicione chat ao vivo à sua aplicação React Native existente, ou até construa uma rede social!
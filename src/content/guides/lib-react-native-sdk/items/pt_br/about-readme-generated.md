Esta biblioteca é uma implementação completa em react-native do [FastComments](https://fastcomments.com).

Suporta comentários em tempo real, chat, threads, emoticons, notificações, SSO, skins e customização completa passando um objeto stylesheet. Todos os assets
também podem ser personalizados, e ela suporta alternar diferentes assets com base no modo escuro.

O benefício desta biblioteca é que ela é mais flexível do que o `fastcomments-react-native` wrapper. Os comentários são renderizados com componentes nativos em vez de dentro de uma webview.

Tudo roda no backend do FastComments, então você só precisa incorporar a UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Veja [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) para mais exemplos.

Adicione chat ao vivo à sua aplicação React Native existente, ou até construa uma rede social!
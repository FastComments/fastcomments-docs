Uma grande vantagem do SSR é que o JavaScript não é necessário. Por causa disso, aplicações podem ser construídas para parecerem "mais leves" em muitos casos de uso.

Além disso, o SSR pode ser usado como fallback caso o usuário tenha o JavaScript desabilitado. Dessa forma, as threads de comentários ainda podem ser renderizadas, e o usuário
pode ainda responder aos comentários.

FastComments já está bem otimizado, então na maioria dos casos o SSR não é necessário. No entanto, algumas comunidades online têm usuários que não usam JavaScript, ou desativá-lo
é a prática preferida. É aí que o SSR do FastComments pode ser útil.

No entanto, uma grande desvantagem do SSR é ter que recarregar a página para pequenas mudanças de estado.
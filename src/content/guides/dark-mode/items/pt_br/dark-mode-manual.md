### Para desenvolvedores - Forçar a desativação do modo escuro

Forçar a desativação do modo escuro pode ser feito passando `hasDarkBackground` como `false` na configuração do widget. Isso funciona para as bibliotecas VanillaJS, Angular, React, Vue e React Native.

Cada biblioteca tem uma pasta `examples` no [GitHub](https://github.com/fastComments/) que contém exemplos de como usar o modo escuro.

### Forçar a ativação do modo escuro

Podemos forçar o modo escuro a estar sempre ativado definindo `hasDarkBackground` como `true`.

Também podemos fazer isso através da interface de personalização do widget [aqui](https://fastcomments.com/auth/my-account/customize-widget).

Em `Base Theme` basta selecionar `Force Dark Mode`.

### Widget VanillaJS - Atualizando o modo escuro

A maneira mais fácil de atualizar o modo escuro é percorrer todas as instâncias do widget na página e atualizar sua configuração:

[inline-code-attrs-start title = 'VanillaJS - Dark Mode Toggle'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    let isDarkMode = somehowDetermineIfDarkModeEnabled();
    for (const instanceWrapped of window.fcUIInstances) {
        if (instanceWrapped.targetElement) {
            const config = instanceWrapped.config;
            config.hasDarkBackground = isDarkMode;
            instanceWrapped.instance.update(config)
        }
    }
[inline-code-end]

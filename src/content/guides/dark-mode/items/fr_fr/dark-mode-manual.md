### Pour les développeurs - Forcer la désactivation du mode sombre

Forcer la désactivation du mode sombre peut être fait en passant `hasDarkBackground` comme `false` dans la configuration du widget. Cela fonctionne pour les bibliothèques VanillaJS, Angular, React, Vue et React Native.

Chaque bibliothèque a un dossier `examples` sur [GitHub](https://github.com/fastComments/) qui contient des exemples sur comment utiliser le mode sombre.

### Forcer l'activation du mode sombre

Nous pouvons forcer le mode sombre à être toujours activé en définissant `hasDarkBackground` à `true`.

Nous pouvons également le faire via l'interface de personnalisation du widget [ici](https://fastcomments.com/auth/my-account/customize-widget).

Sous `Thème de base`, sélectionnez simplement `Forcer le mode sombre`.

### Widget VanillaJS - Mise à jour du mode sombre

La façon la plus simple de mettre à jour le mode sombre est de parcourir toutes les instances du widget sur la page et de mettre à jour leur configuration:

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

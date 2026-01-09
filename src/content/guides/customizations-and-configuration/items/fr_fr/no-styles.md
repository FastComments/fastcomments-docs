[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

Pour des projets de personnalisation de style plus importants, il peut être souhaitable de partir d'une page blanche et de ne pas utiliser du tout le style par défaut.

Il est possible de supprimer tous les styles par défaut en définissant le paramètre **noStyles** sur true, comme suit :

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Cela peut être personnalisé sans code, depuis la page de personnalisation du widget, sous Options avancées :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]

---
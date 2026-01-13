[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Когато потребител публикува коментар с FastComments за първи път, ще се опитаме да изтеглим техния аватар от <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Ако обаче не намерим аватар, или потребителят никога не е задавал такъв в своя профил, показваме статично изображение по подразбиране като аватар.

За да посочите собствено статично изображение за аватар, може да използвате настройката *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Това може да се направи и без код. В страницата за персонализиране на уиджета вижте секцията "Аватар по подразбиране".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Обърнете внимание, че задаването на аватар за конкретен потребител, например при SSO, е разгледано в отделна секция.
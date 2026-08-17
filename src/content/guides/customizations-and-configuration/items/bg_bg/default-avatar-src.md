[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Когато потребител коментира с FastComments за първи път, ще се опитаме да извлечем неговия аватар от <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

Въпреки това, ако не намерим аватар или потребителят никога не зададе такъв в своя акаунт, ще покажем статично изображение за стандартен аватар.

За да зададете собствено статично изображение за аватар, можете да използвате настройката *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Презаписване на стандартния аватар'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета, вижте секцията "Default Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Секция за стандартния аватар на страницата за персонализиране на уиджета, където задавате URL на резервния аватар'; title='Персонализиране на стандартния аватар' app-screenshot-end]

Обърнете внимание, че определянето на аватар за конкретен потребител, например чрез SSO, е разгледано в отделна секция.
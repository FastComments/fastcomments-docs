[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Kiedy użytkownik po raz pierwszy komentuje przy użyciu FastComments, spróbujemy pobrać jego awatar z <a href="https://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Jednakże, jeśli nie znajdziemy awatara lub użytkownik nigdy nie ustawi go w swoim koncie, wyświetlimy statyczny domyślny obraz awatara.

Aby określić własny statyczny obraz awatara, możemy użyć ustawienia *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Można to również zrobić bez kodu. Na stronie dostosowywania widgetu, zobacz sekcję „Domyślny awatar”.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Sekcja Domyślnego awatara na stronie dostosowywania widgetu, gdzie ustawiasz adres URL awatara zapasowego'; title='Dostosowywanie domyślnego awatara' app-screenshot-end]

Zauważ, że definiowanie awatara dla konkretnego użytkownika, na przykład przy użyciu SSO, jest opisane w osobnej sekcji.
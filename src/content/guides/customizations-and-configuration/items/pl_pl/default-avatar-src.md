[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Gdy użytkownik po raz pierwszy komentuje za pomocą FastComments, spróbujemy pobrać jego awatar z <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Jeśli jednak nie znajdziemy awatara lub użytkownik nigdy go nie ustawił w swoim koncie, wyświetlamy statyczny domyślny obraz awatara.

Aby określić własny statyczny obraz awatara, możemy użyć ustawienia *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Można to również zrobić bez użycia kodu. Na stronie dostosowywania widżetu zobacz sekcję "Domyślny Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Należy pamiętać, że definiowanie awatara dla konkretnego użytkownika, na przykład w przypadku SSO, jest omówione w osobnej sekcji.

---
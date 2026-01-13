[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Når en bruger kommenterer med FastComments for første gang, vil vi forsøge at hente deres avatar fra <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Hvis vi derimod ikke finder en avatar, eller brugeren aldrig sætter en i deres konto, viser vi et statisk standard-avatarbillede.

For at angive dit eget statiske avatarbillede kan vi bruge indstillingen *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Dette kan også gøres uden kode. På widgettilpasningssiden, se afsnittet "Standard-avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Bemærk, at det at definere avataren for en bestemt bruger, som f.eks. med SSO, er dækket i sit eget afsnit.

---
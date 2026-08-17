[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Når en bruger kommenterer med FastComments for første gang, vil vi forsøge at hente deres avatar fra <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

Hvis vi dog ikke finder en avatar, eller brugeren aldrig indstiller en i deres konto, viser vi et statisk standardavatarbillede.

For at angive dit eget statiske avatarbillede kan du bruge indstillingen *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Dette kan også gøres uden kode. På widget-tilpasningssiden, se sektionen "Default Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Standard Avatar sektion på widget-tilpasningssiden, hvor du angiver fallback-avatarbilledets URL'; title='Tilpasning af standardavatar' app-screenshot-end]

Bemærk, at definition af avataren for en bestemt bruger, som med SSO, er dækket i sin egen sektion.
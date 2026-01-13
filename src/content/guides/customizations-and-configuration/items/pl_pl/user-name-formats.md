Domyślnie FastComments pokaże nazwę użytkownika tak, jak ją wprowadził, lub tak, jak została przekazana przez SSO.

Jednak może być pożądane zamaskowanie lub wyświetlenie nazwy użytkownika w inny sposób. Na przykład, jeśli nazwa użytkownika to Allen Rex, być może
chcesz pokazywać tylko "Allen R.".

Można to zrobić bez kodu w Widget Customization UI, w ustawieniu o nazwie `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Dostępne formaty to:

- Capitalize (wyświetla Example User jako Example User)
- Last Initial (wyświetla Example User jako Example U.)
- All Initials (wyświetla Example User jako E. U.)
- Pokaż "Anonim"

Efekt tej zmiany jest natychmiastowy. Użytkownicy nadal będą widzieć swoją pełną nazwę użytkownika na górze obszaru komentarzy, dla siebie, ale ich komentarze będą pokazywać
zmodyfikowaną nazwę użytkownika.

Nazwy użytkowników są maskowane po stronie serwera w celu ochrony użytkowników.
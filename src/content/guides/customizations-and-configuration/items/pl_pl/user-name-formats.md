Domyślnie FastComments wyświetla nazwę użytkownika tak, jak została wprowadzona, lub jak została przekazana nam przez SSO.

Jednakże może być pożądane ukrycie lub wyświetlenie nazwy użytkownika w inny sposób. Na przykład, jeśli nazwa użytkownika to Allen Rex, może chcesz wyświetlać tylko „Allen R.”.

Można to zrobić bez kodu w interfejsie dostosowywania widgetu, w ustawieniu o nazwie `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Dropdown Formatu Nazwy Komentującego otwarty z opcjami takimi jak Capitalize, Last Initial i All Initials'; title='Zmień format nazwy' app-screenshot-end]

Dostępne formaty są:

- Capitalize (wyświetla przykładowego użytkownika jako Example User)
- Last Initial (wyświetla Example User jako Example U.)
- All Initials (wyświetla Example User jako E. U.)
- Show "Anonymous"

Efekt zmiany jest natychmiastowy. Użytkownicy nadal zobaczą swoją pełną nazwę użytkownika na górze obszaru komentarza, dla siebie, ale ich komentarze będą wyświetlać zmodyfikowaną nazwę użytkownika.

Nazwy użytkowników są maskowane po stronie serwera w celu ochrony użytkowników.
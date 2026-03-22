Istnieją dwa sposoby zablokowania użytkowników przed komentowaniem na Twojej stronie za pomocą FastComments.

Pierwszy to, jeśli znasz już ich adres e-mail, możesz go wpisać na stronie <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">zbanowani użytkownicy</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Do tej strony można przejść poprzez Moderate Comments -> Banned Users

Gdy zamierzamy zbanować użytkownika, możemy wybrać typ: albo Permanent albo Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Drugim sposobem zbanowania użytkownika jest kliknięcie przycisku ban, który znajduje się przy każdym komentarzu na stronie Comment Moderation.

Po kliknięciu przycisku ban pojawią się opcje, w których możemy określić typ i czas trwania bana.

### Aliasy adresów e-mail

Podczas blokowania użytkownika po adresie e-mail FastComments automatycznie ignoruje aliasy z użyciem `+`. Na przykład zbanowanie `user+alias@gmail.com` spowoduje również zablokowanie `user@gmail.com` oraz każdej innej wariacji z `+`, takiej jak `user+other@gmail.com`.

### Shadow Bany

Shadow-ban to rodzaj blokady, która sprawia, że wygląda, iż komentarz lub głos użytkownika został zapisany pomyślnie, podczas gdy w rzeczywistości nie został. Może to być pożądane w niektórych sytuacjach.

### Blokowanie przez adres IP

O ile tenant nie zechce zrezygnować, FastComments obsługuje blokowanie po IP poprzez przechowywanie zhaszowanej wersji adresu IP komentującego.
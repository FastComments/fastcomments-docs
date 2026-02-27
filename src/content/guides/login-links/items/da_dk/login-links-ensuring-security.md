Da login-links i det væsentlige er adgangskoder, tager vi sikkerheden meget alvorligt.

Alle login-links i vores system er sat til at udløbe efter en bestemt tidsperiode, og vi har også mekanismer på plads til at opdage
gætning af et login-link. Nogle login-links er delt op i flere adgangskoder, og hvis én bliver gættet,
vil den anden blive ugyldiggjort.

### Sikkerhed sammenlignet med adgangskoder

Med de fleste systemer, der kræver en adgangskode, kan du gå gennem en Glemt adgangskode-mekanisme
hvis du har brugerens e-mail. Det betyder, at hvis du har adgang til brugerens e-mailkonto,
er det ligegyldigt, om systemet under angreb bruger adgangskoder eller magiske links.

### Nye IP-login-advarsler

Når et login sker fra en IP-adresse, som ikke tidligere er set for en given konto, sender FastComments en sikkerhedsadvarsels-e-mail
med den omtrentlige placering og IP-adressen. Dette hjælper brugere med at opdage uautoriseret adgang. Bemærk, at FastComments ikke gemmer
rå IP-adresser — kun en obfuskeret form gemmes af sikkerhedsmæssige årsager.

### Backup-e-mail til kontogendannelse

Hvis du mister adgangen til din primære e-mail, kan du bruge en verificeret backup-e-mail til at gendanne din konto. Din backup-e-mail fungerer
med alle loginflows. Du kan indtaste den på siden for glemt brugernavn, bruge den med login med magisk link, eller skrive den i
brugernavn/e-mail-feltet ved adgangskode-login.

For at opsætte en backup-e-mail, gå til [Account Details](https://fastcomments.com/auth/my-account/edit-details) og klik
**Definér en backup-e-mail**. Din backup-e-mail bruges kun til kontogendannelse og vil ikke modtage notifikationer.

### Sikkerhed sammenlignet med MFA

Login-links er mindre sikre end MFA. FastComments understøtter nu tofaktorgodkendelse (2FA)
for administrator-konti for at give forbedret sikkerhed. Når 2FA er aktiveret, er det påkrævet selv ved brug af login-links.
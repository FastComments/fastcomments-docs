Da login-links i det væsentlige er adgangskoder, tager vi sikkerheden meget alvorligt.

Alle login-links i vores system er indstillet til at udløbe efter en vis periode, og vi har også mekanismer på plads til at opdage
gætning af et login-link. Nogle login-links er delt op i flere adgangskoder, og hvis én gættet,
vil den anden blive ugyldiggjort.

### Sikkerhed sammenlignet med adgangskoder

Med de fleste systemer, der kræver en adgangskode, kan du gå igennem funktionen "Glemt adgangskode"
hvis du har brugerens e-mail. Det betyder, at hvis du har adgang til brugerens e-mailkonto,
er det ligegyldigt, om det system, der er under angreb, bruger adgangskoder eller magic links.

### Nye IP-login-advarsler

Når der sker et login fra en IP-adresse, der ikke tidligere er blevet set for en given konto, sender FastComments en sikkerhedsadvarsel via e-mail
med den omtrentlig placering og IP-adressen. Dette hjælper brugere med at opdage uautoriseret adgang. Bemærk, at FastComments ikke gemmer
rå IP-adresser — kun en sløret form gemmes af sikkerhedsmæssige årsager.

### Sikkerhed sammenlignet med MFA

Login-links er mindre sikre end MFA. FastComments understøtter nu to-faktor-autentificering (2FA)
for administrator-konti for at give øget sikkerhed. Når 2FA er aktiveret, er det påkrævet, selv ved brug af login-links.

---
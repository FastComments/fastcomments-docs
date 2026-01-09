Da login-links i bund og grund er adgangskoder, tager vi sikkerheden meget alvorligt.

Alle login-links i vores system er indstillet til at udløbe efter en vis periode, og vi har også mekanismer på plads til at opdage
gætteri af et login-link. Nogle login-links er opdelt i flere adgangskoder, og hvis én bliver gættet,
vil den anden blive ugyldiggjort.

### Sikkerhed sammenlignet med adgangskoder

Med de fleste systemer, der kræver en adgangskode, kan du benytte en "Glemt adgangskode"-mekanisme
hvis du har brugerens e-mail. Det betyder, at hvis du har adgang til brugerens e-mailkonto,
betyder det ikke noget, om systemet under angreb bruger adgangskoder eller magiske links.

### Sikkerhed sammenlignet med MFA

Login-links er mindre sikre end MFA. FastComments understøtter nu tofaktorgodkendelse (2FA)
for admin-konti for at give øget sikkerhed. Når 2FA er aktiveret, kræves det også ved brug af login-links.
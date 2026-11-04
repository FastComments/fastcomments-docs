For udviklere, som du måske ikke ønsker skal være `Administrators`, kan du overveje at oprette en `Administrator`-bruger
med følgende rettigheder:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Dette sæt af rettigheder giver en udvikler alt, hvad de behøver for at sætte FastComments op, samt
det indblik i systemet, der kræves for at sikre, at det fungerer.

Begrundelsen for disse rettigheder er som følger:

1. **Analytics Admin**: Dette kan bruges til at fejlsøge brugen af FastComments.
2. **Customizations Admin**: Dette vil være nødvendigt for at anvende tilpasset styling på kommentar-widget'en.
3. **Data Management Admin**: Dette vil være nødvendigt for at administrere imports og exports samt opsætte webhooks.
4. **Comment Moderation Admin**: Dette vil være nødvendigt for at se kommentardata, i det mindste under opsætningen.
5. **API/SSO Admin**: Dette giver dem mulighed for at hente API-nøglerne direkte fra vores platform. Vi anser
dette for mere sikkert end en `Administrator`, der kopierer det for dem og sender API Secret via e-mail, hvilket
   måske ikke er særlig sikkert.
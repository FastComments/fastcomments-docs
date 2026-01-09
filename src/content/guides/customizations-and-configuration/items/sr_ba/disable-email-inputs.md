---
Kada korisnici komentarišu, a nisu prijavljeni, zatražiće im se da navedu svoju email adresu.

To će za tog korisnika kreirati "unverified session", i tražićemo od njih da potvrde tu sesiju putem emaila.

Za neke sajtove ili aplikacije poželjno je ne tražiti email od korisnika prilikom komentarisanja ili glasanja.

Omogućavanje anonimnog komentarisanja čini polje za unos emaila opcionalnim. Međutim, možemo ga potpuno onemogućiti. Prvo omogućite
anonimno komentarisanje, i tada će se pojaviti opcija za onemogućavanje polja za unos emaila.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments', '.disable-email-inputs']; selector = '.disable-email-inputs'; title='Disable Email Inputs' app-screenshot-end]

Sa ovim uključenim, polja za email uopće se neće prikazivati u svim našim proizvodima za komentarisanje.

Imajte na umu da će, sa ovom konfiguracijom, svi komentari biti nepotvrđeni osim ako korisnik ne napravi nalog i ne prijavi se na
https://fastcomments.com.

Možda ćete htjeti razmotriti [onemogućavanje oznake 'nepotvrđeno'](/guide-customizations-and-configuration.html#disable-unverified-label).

---
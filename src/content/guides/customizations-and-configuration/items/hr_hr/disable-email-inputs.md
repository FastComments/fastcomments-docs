Kada korisnici komentiraju, a nisu prijavljeni, bit će im zatraženo da navedu svoju e-poštu.

To će stvoriti "nepotvrđenu sesiju" za tog korisnika, i zatražit ćemo od njih da potvrde tu sesiju putem e-pošte.

Za neke web-lokacije ili aplikacije poželjno je ne tražiti od korisnika njihovu e-poštu prilikom komentiranja ili glasovanja.

Omogućavanje anonimnog komentiranja čini polje za unos e-pošte opcionalnim. Međutim, možemo ga u potpunosti onemogućiti. Prvo, omogućite
anonymous commenting, and then the option to disable the email input fields will appear.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments', '.disable-email-inputs']; selector = '.disable-email-inputs'; title='Disable Email Inputs' app-screenshot-end]

Kad je ovo uključeno, polja za e-poštu se uopće neće prikazivati u svim našim proizvodima za komentiranje.

Imajte na umu da će, s ovom konfiguracijom, svi komentari biti nepotvrđeni osim ako korisnik ne stvori račun i ne prijavi se na
https://fastcomments.com.

Možda biste htjeli razmotriti [onemogućavanje oznake 'nepotvrđeno'](/guide-customizations-and-configuration.html#disable-unverified-label).
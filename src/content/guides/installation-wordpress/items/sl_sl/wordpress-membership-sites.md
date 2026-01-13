FastComments deluje s spletnimi mesti, namenjenimi le članom, z uporabo t.i. SSO, oziroma enotne prijave. Vaši člani se prijavijo na vaše WordPress spletno mesto, vendar
se jim ni treba ukvarjati z ustvarjanjem računa pri FastComments ali prijavo z družbenimi omrežji, da bi komentirali. Če so vaši člani (vključno z skrbniki) prijavljeni v
vaše WordPress spletno mesto, bodo lahko komentirali. Vaši skrbniki in moderatorji bodo lahko tudi neposredno moderirali komentarje iz vaših WordPress objav na blogu.

<sup>(Optional)</sup> Ne pozabite tudi dodati vaših administratorjev na [User & Administrators](https://fastcomments.com/auth/my-account/users) in moderatorjev na [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
za izboljšanje njihove izkušnje in omogočanje sledenja statistike za moderatorje.

SSO lahko omogočite tako, da odprete nadzorno ploščo vtičnika in kliknete "SSO Settings".

Najprej boste morali omogočiti funkcijo "Anyone can Register" vašega spletnega mesta.

Vse uporabniške informacije se nemoteno in varno prenesejo v FastComments vsakič, ko uporabnik ogleda nit komentarjev preko [HMAC](https://en.wikipedia.org/wiki/HMAC).

Ni začetne ali neprekinjene sinhronizacije, ki bi jo bilo treba zagnati za kopiranje vaših članov na strežnike FastComments. To se samodejno izvede, ko si ogledajo niti komentarjev z odpiranjem objave na blogu.

## Imena in avatarji

Vtičnik bo samodejno posodobil prikazno ime uporabnika in avatar pri vseh njihovih komentarjih znotraj FastComments, ko si ogledajo
katero koli nit komentarjev. Avatarji so podprti preko Gravatar ali katerega koli vtičnika za upravljanje avatarjev znotraj WordPress, saj bo vtičnik poklical `get_avatar_url`.
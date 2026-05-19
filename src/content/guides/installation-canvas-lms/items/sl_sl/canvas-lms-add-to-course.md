#### Kako se komentarji prikazujejo v vaših predmetih

Ko je LTI integracija omogočena in je nameščena zunanja aplikacija, FastComments deluje samodejno glede na postavitve, ki ste jih konfigurirali:

#### Pogled naloge

Če je postavitev **Pogled naloge** omogočena, se komentarji samodejno prikažejo pod vsako nalogo v predmetu. Študenti in inštruktorji vidijo razvejano nit komentarjev, ko si ogledajo nalogo — za vsako nalogo ni potrebna dodatna nastavitev.

Vsaka naloga dobi svojo ločeno nit komentarjev.

#### Gumb urejevalnika bogate vsebine

Če je postavitev **Gumb urejevalnika** omogočena, lahko inštruktorji vgradijo FastComments v katerokoli vsebino, ki uporablja Urejevalnik bogate vsebine:

1. Uredite **Stran**, **Kviz** ali **Obvestilo**.
2. V orodni vrstici Urejevalnika bogate vsebine kliknite gumb **FastComments**.
3. FastComments se samodejno vgradi v vsebino.
4. Shrani stran.

Ko si študenti ogledajo stran, se naloži vgrajen pripomoček FastComments z nitjo komentarjev, ki je edinstvena za to stran.

#### Samodejni SSO

V obeh postavitvah so študenti samodejno prijavljeni prek svojega računa Canvas. Imena, e-poštni naslovi in avatarji se sinhronizirajo preko LTI launch, ni potrebna ločena prijava.

#### Omejitev javnega dostopa (priporočeno)

Privzeto so podatki komentarjev FastComments javno berljivi. Kdor koli, ki lahko uganedi URL niti ali API končno točko, si lahko ogleda njene komentarje, tudi izven Canvasa. Za razprave v predmetu boste skoraj zagotovo želeli omejiti ogled samo na vpisane študente.

Odprite svojo <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stran za prilagoditev vtičnika</a> in ustvarite pravilo z omogočeno možnostjo **Zahtevaj SSO za ogled komentarjev**, nato nastavite varnostno raven na **Varni SSO**, tako da se niti lahko naložijo samo preko podpisanega LTI launch.

Oglejte si [Zaščita niti komentarjev z enkratno prijavo](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za celoten vodnik, vključno s tem, kako omejiti pravilo na eno domeno ali stran.

---
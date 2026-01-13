---
Privzeto lahko vsak uporabnik odda do `5 comments` v isti minuti.

To se spremlja po user id, anon user id in ip address (hashed).

To lahko prilagodite brez kode na strani za prilagajanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Upoštevajte, da če uporabljate comment creation API, boste morda želeli v zahtevi do našega strežnika posredovati izvorni `ip` naslov uporabnika, tako da bo omejevanje hitrosti uporabljeno na posameznega uporabnika in ne globalno za vaš račun.

---
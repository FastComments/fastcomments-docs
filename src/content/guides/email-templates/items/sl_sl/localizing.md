---
FastComments je lokalizirana platforma. Vsi naši widgeti, e-pošte in obvestila so lokalizirani.

Lokalizirano pomeni, da prikažemo drugačen jezik in oblikovanje glede na lokacijo uporabnika
in njegov želeni jezik. To določimo na podlagi informacij, ki nam jih posreduje uporabnikov brskalnik.

Besedilo v e-pošti lahko prilagodimo tako, da gremo na zavihek `Translations`, izberemo `Locale`
in uredimo besedilo. Besedilo, spremenjeno glede na privzeto, je v uporabniškem vmesniku označeno. Lahko
preklapljate med lokalami in na koncu shranite, brez izgube sprememb.

Do lokaliziranega besedila dostopamo preko objekta `TEXT`, na primer: `<%= TEXT.INTRO %>`.

### Opomba za SSO

Za integracije SSO, če `locale` ni določen, se bo ta posodobil vsakič, ko uporabnik
dostopa do pripomočka za komentarje z drugo lokalizacijo. To pomeni, da se njihova jezikovna preference
samodejno posodobi in bodo prihodnja e-poštna sporočila poslana v tej lokalizaciji.

To je mogoče nastaviti tudi ročno z vključitvijo `locale` v SSO payload.

---
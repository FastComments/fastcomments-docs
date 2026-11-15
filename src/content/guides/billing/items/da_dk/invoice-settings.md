Siden [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), som nås med knappen `Invoice Settings` på Billing Info, styrer hvad der vises på dine fakturaer, og hvor de sendes hen. Kun `Account Owner` eller en `Billing Admin` kan åbne den.

- `Company Name`: navnet på din konto, som det vises i dashboardet, i e-mails og i teaminvitationer. Dette er ikke faktureringsnavnet.
- `Billing Email`: hvor fakturaer og faktureringsnotifikationer sendes. Sæt denne til en økonomi- eller regnskabsmailbox, hvis kontoejeren ikke skal modtage dem. Den ændrer ikke login-e-mailen for nogen bruger.
- `Invoice Currency`: den valuta, dine fakturaer udstedes i. Se [Currencies](#currencies).
- `Billing Address`: `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` og `Country`, som trykkes på fakturaer. De samme felter vises på Billing Info-formularen, når du gemmer et kort; begge skriver til samme sted.
- `VAT / Tax ID`: valgfri. Trykkes på dine fakturaer.

Klik på `Save Changes`. Siden bekræfter med `Invoice settings updated successfully.`, og ændringerne gælder for den næste faktura. Eksisterende fakturaer ændres ikke.

### VAT And Tax IDs

Vælg dit fakturerings-`Country`, før du indtaster ID'et, fordi landet afgør, hvilken slags ID det er: et EU-momsnummer for enhver EU-medlemsstat, et britisk momsnummer for Storbritannien, et ABN for Australien, et GST/HST-nummer for Canada, et EIN for USA og så videre for de andre lande, som Stripe understøtter.

ID'et valideres, når du gemmer. Hvis det ikke er gyldigt for det valgte land, siger siden det, og ID'et gemmes ikke, mens dine andre indstillinger gør. Hvis dit land ikke er et, vi kan registrere et skatte-ID for, fortæller siden dig det også.

Hvis du har en EU-faktureringsadresse, udstedes Flex-fakturaer med 0% moms som en reverse charge-transaktion, og dit momsnummer vises på dem, når det er gemt, så din revisor kan håndtere momsen på din side.
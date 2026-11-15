La page [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), accessible via le bouton `Invoice Settings` sur la page Billing Info, contrôle ce qui apparaît sur vos factures et où elles sont envoyées. Seul l'`Account Owner` ou un `Billing Admin` peut l'ouvrir.

- `Company Name` : le nom de votre compte tel qu'affiché dans le tableau de bord, les e-mails et les invitations d'équipe. Ce n'est pas le nom de facturation.
- `Billing Email` : l'adresse à laquelle les factures et les notifications de facturation sont envoyées. Définissez-la sur une boîte mail financière ou comptable si le propriétaire du compte ne doit pas les recevoir. Cela ne modifie pas l'e-mail de connexion d'un utilisateur.
- `Invoice Currency` : la devise dans laquelle vos factures sont émises. Voir [Currencies](#currencies).
- `Billing Address` : le `Name`, l'`Address`, la `City`, le `State / Province / Region`, le `Postal Code` et le `Country` imprimés sur les factures. Les mêmes champs apparaissent sur le formulaire Billing Info lorsque vous enregistrez une carte ; les deux écrivent au même endroit.
- `VAT / Tax ID` : facultatif. Imprimé sur vos factures.

Cliquez sur `Save Changes`. La page confirme avec `Invoice settings updated successfully.` et les modifications s'appliquent à la prochaine facture. Les factures existantes ne sont pas modifiées.

### Numéros de TVA et identifiants fiscaux

Sélectionnez votre `Country` de facturation avant de saisir l'identifiant, car le pays détermine le type d'identifiant : un numéro de TVA UE pour tout État membre de l'UE, un numéro de TVA britannique pour le Royaume-Uni, un ABN pour l'Australie, un numéro de GST/HST pour le Canada, un EIN pour les États-Unis, et ainsi de suite pour les autres pays pris en charge par Stripe.

L'identifiant est validé lors de l'enregistrement. S'il n'est pas valide pour le pays sélectionné, la page l'indique et l'identifiant n'est pas enregistré, tandis que vos autres paramètres le sont. Si votre pays ne fait pas partie de ceux pour lesquels nous pouvons enregistrer un identifiant fiscal, la page vous le signale également.

Si vous avez une adresse de facturation dans l'UE, les factures Flex sont émises avec 0 % de TVA dans le cadre d'une transaction d'autoliquidation, et votre numéro de TVA y apparaît une fois enregistré, afin que votre comptable puisse gérer la TVA de votre côté.
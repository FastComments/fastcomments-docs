The [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings) page, reached with the `Invoice Settings` button on Billing Info, controls what appears on your invoices and where they are sent. Only the `Account Owner` or a `Billing Admin` can open it.

- `Company Name`: the name of your account as shown in the dashboard, emails, and team invites. This is not the billing name.
- `Billing Email`: where invoices and billing notifications are sent. Set this to a finance or accounts mailbox if the account owner should not receive them. It does not change the login email of any user.
- `Invoice Currency`: the currency your invoices are issued in. See [Currencies](#currencies).
- `Billing Address`: the `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code`, and `Country` printed on invoices. The same fields appear on the Billing Info form when you save a card; both write to the same place.
- `VAT / Tax ID`: optional. Printed on your invoices.

Click `Save Changes`. The page confirms with `Invoice settings updated successfully.` and the changes apply to the next invoice. Existing invoices are not changed.

### VAT And Tax IDs

Select your billing `Country` before entering the ID, because the country decides what kind of ID it is: an EU VAT number for any EU member state, a UK VAT number for the United Kingdom, an ABN for Australia, a GST/HST number for Canada, an EIN for the United States, and so on for the other countries Stripe supports.

The ID is validated when you save. If it is not valid for the selected country, the page says so and the ID is not saved, while your other settings are. If your country is not one we can record a tax ID for, the page tells you that too.

If you have an EU billing address, Flex invoices are issued with 0% VAT as a reverse charge transaction, and your VAT number appears on them once saved, so your accountant can handle the VAT on your side.
